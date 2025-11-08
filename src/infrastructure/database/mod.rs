use crate::util::{self};

use postgres::{Client, NoTls, types::to_sql_checked};
use std::collections::HashMap;

pub mod migration;

#[derive(Debug)]
pub struct DatabaseError {
    kind: DatabaseErrorKind,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error in database layer")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DatabaseErrorKind::NotConnected(e) => Some(e),
            DatabaseErrorKind::AlreadyExists(e) => Some(e),
            DatabaseErrorKind::InjectionAttempt => None,
            DatabaseErrorKind::TableDoesNotExist(e) => Some(e),
            DatabaseErrorKind::UnknownType(e) => Some(e),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[allow(dead_code)]
enum DatabaseErrorKind {
    NotConnected(postgres::Error),
    AlreadyExists(postgres::Error),
    InjectionAttempt,
    TableDoesNotExist(postgres::Error),
    UnknownType(postgres::Error),
}

impl std::fmt::Display for DatabaseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotConnected(_e) => f.write_str("connection couldn't be established"),
            Self::AlreadyExists(_e) => f.write_str("the dataset does already exist"),
            Self::InjectionAttempt => f.write_str("this might be a sql injection"),
            Self::TableDoesNotExist(_e) => f.write_str("the table already exists"),
            Self::UnknownType(_e) => {
                f.write_str("the data type is not known to this application, implement it")
            }
        }
    }
}

impl std::error::Error for DatabaseErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

pub struct Field {
    pub name: String,
    pub sql_type: String,
}

impl Field {
    fn new(name: String, sql_type: String) -> Self {
        Self { name, sql_type }
    }
}

pub struct Query {}

pub struct Connection {
    query: String,
    client: Client,
}

impl Connection {
    pub fn new(client: Client) -> Self {
        Self {
            query: String::from("query"),
            client,
        }
    }
}

pub trait ActiveConnection {
    fn insert(&mut self, fields: &Vec<Field>, table_name: &str) -> Result<(), DatabaseError>;
    fn get_schema(
        &mut self,
        table_name: &str,
    ) -> Result<HashMap<String, serde_json::Value>, DatabaseError>;
    #[allow(dead_code)]
    fn execute(&mut self, query: Query);
    #[allow(dead_code)]
    fn close();
}

impl ActiveConnection for Connection {
    fn insert(&mut self, fields: &Vec<Field>, table_name: &str) -> Result<(), DatabaseError> {
        if util::is_sql(table_name) {
            for field in fields {
                println!("name: {}, sql_type: {}", field.name, field.sql_type)
            }
            Err(DatabaseError {
                kind: DatabaseErrorKind::InjectionAttempt,
            })
        } else {
            self.query = format!("INSERT {table_name}");
            let params = String::from("");
            let _ = self.client.query(&self.query, &[&params]);
            Ok(())
        }
    }

    fn get_schema(
        &mut self,
        table_name: &str,
    ) -> Result<HashMap<String, serde_json::Value>, DatabaseError> {
        let query = "select * 
        from information_schema.columns 
        where table_name = $1 and table_schema = 'public'";
        let result = self
            .client
            .query(query, &[&table_name])
            .map_err(|e| DatabaseErrorKind::TableDoesNotExist(e))
            .map_err(|kind| DatabaseError { kind });
        let mut schema: HashMap<String, serde_json::Value> = HashMap::new();
        for i in result.unwrap() {
            schema.insert(
                i.try_get("column_name")
                    .map_err(|e| DatabaseErrorKind::UnknownType(e))
                    .map_err(|kind| DatabaseError { kind })?,
                parse_field(
                    i.try_get("data_type")
                        .map_err(|e| DatabaseErrorKind::UnknownType(e))
                        .map_err(|kind| DatabaseError { kind })?,
                ),
            );
        }
        Ok(schema)
    }

    fn execute(&mut self, _query: Query) {
        todo!()
    }

    fn close() {
        todo!()
    }
}

fn parse_field(ty: &str) -> serde_json::Value {
    todo!()
}

pub fn connect() -> Result<Connection, DatabaseError> {
    // pub fn new() -> Result<Connection, DatabaseException> {
    match postgres_client() {
        Ok(client) => Ok(Connection::new(client)),
        Err(e) => {
            // match e.into_source() {
            //     Ok(e2) => e2.
            // }
            // let s = e.kind().to_string();
            // println!("{}");
            Err(DatabaseError {
                kind: DatabaseErrorKind::NotConnected(e),
            })
        } // Err(e) => Err("Something went wrong!".into()),
    }
}

fn postgres_client() -> Result<Client, postgres::Error> {
    // fn post_gres_client() -> Result<Client, DatabaseException> {
    // fn persist(data_set: DataSet) -> Result<(), Error> {
    let _prot = "postgres";
    let user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let pw = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let host = "localhost";
    let db = "postgres";
    let _schema = "bc";
    let connection_str =
        // format!("host={host} user={user} password={pw} dbname={db} schema={schema}");
        format!("host={host} user={user} password={pw} dbname={db}");

    let mut client = Client::connect(&connection_str, NoTls)?;

    // match (client.query("SELECT id FROM postgres.public.account", &[])) {
    //     Ok(q) => {
    //         let row = q.get(0);
    //         let v: String = row.unwrap().get(0);
    //         print!("{v}")
    //     }
    //     // Err(_e) => Err(Box::from(DatabaseException)),
    //     Err(_e) => Err(DatabaseException),
    // }
    let query = client.query("SELECT id, balance FROM public.account", &[]);
    match query {
        Ok(_q) => iter_row(_q)?,
        Err(e) => {
            if let Some(err) = e.as_db_error() {
                eprintln!("{}", err.message());
            }
        }
    }
    Ok(client)
    // match client {
    //     Ok(c) => Ok(c),
    //     Err(_e) => Err(DatabaseException {}),
    // }
}

fn iter_row(query_result: Vec<postgres::Row>) -> Result<(), postgres::Error> {
    Ok(for row in query_result {
        let id: i64 = row.try_get("id")?;
        let balance: f64 = row.try_get("balance")?;
        println!("Account {}: {}", id, balance);
    })
}
