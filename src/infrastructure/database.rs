use crate::util::{self};

use postgres::{Client, NoTls};
pub struct DatabaseError(pub Box<dyn std::error::Error>);
pub struct Row {
    pub label: String,
    pub sql_type: String,
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
            client
        }
    }
}

pub trait ActiveConnection {
    fn insert(&mut self, rows: Vec<Row>, table_name: &str) -> Result<(), DatabaseError>;
    #[allow(dead_code)]
    fn execute(&mut self, query: Query);
    #[allow(dead_code)]
    fn close();
}

impl ActiveConnection for Connection {
    fn insert(&mut self, rows: Vec<Row>, table_name: &str) -> Result<(), DatabaseError> {
        if util::is_sql(table_name) {
            for field in rows {
                println!("label: {}, sql_type: {}", field.label, field.sql_type)
            }
            Err(DatabaseError(Box::from("SQL-like syntax is not allowed")))
        } else {
            self.query = format!("INSERT {table_name}");
            let params = String::from("");
            let _ = self.client.query(&self.query, &[&params]);
            Ok(())
        }
    }
    
    fn execute(&mut self, _query: Query) {
        todo!()
    }
    
    fn close() {
        todo!()
    }
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
            Err(DatabaseError(Box::from(e)))
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

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn vec2_test1() {
        assert_eq!(vec2!("1","2","4").get(2).unwrap(), &String::from("4"));
    }
}