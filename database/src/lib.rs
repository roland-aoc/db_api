pub mod util;
pub mod domain;
use crate::domain::{StructDef, Field};
use std::collections::{HashMap, HashSet, VecDeque};
use crate::util::Conf;

use postgres::{Client, NoTls};
use syn;
use regex::Regex;

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
            NotConnected(e) => Some(e),
            AlreadyExists(e) => Some(e),
            InjectionAttempt => None,
            TableDoesNotExist(e) => Some(e),
            UnknownType(e) => Some(e),
            UnknownInputType(_ty_str) => None,
            ConfigUnavailable(e) => Some(e),
            InvalidQuery(e) => Some(e),
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
    UnknownInputType(String),
    ConfigUnavailable(std::io::Error),
    InvalidQuery(postgres::Error),
}
use DatabaseErrorKind::*;

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
            Self::UnknownInputType(ty_str) => f.write_str(
                format!(
                    "the requested type {:?} is unknown to this application",
                    ty_str
                )
                .as_str(),
            ),
            Self::ConfigUnavailable(_e) => f.write_str("the config fil cannot be read"),
            Self::InvalidQuery(_e) => f.write_str("the query is invalid"),
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

pub struct ParameterizedQuery {
    sql_query: String,
}

impl ParameterizedQuery {
    pub fn new(table_name: String, where_fields: Vec<String>, where_values: Vec<String>) -> Self {
        if where_fields.len() != where_values.len() {
            panic!("number of fields and values must be the same");
        } else if where_fields.is_empty() {
            Self {
                sql_query: format!("SELECT * FROM {}", table_name),
            }
        } else {
            let mut query = format!("SELECT * FROM {} WHERE ", table_name);
            for i in 0..where_fields.len() {
                if i == where_fields.len() - 1 {
                    query.push_str(&format!("{} = '{}'", where_fields[i], where_values[i]));
                } else {
                    query.push_str(&format!("{} = '{}' AND ", where_fields[i], where_values[i]));
                }
            }
            Self { sql_query: query }
        }
    }
}

pub struct Ddl {
    sql_statement: String,
}

impl Ddl {
    pub fn new(
        ddl_type: DdlType,
        table_name: &str,
        fields: &HashMap<&syn::Ident, &syn::Type>,
    ) -> Self {
        let instance = Self {
            sql_statement: match ddl_type {
                DdlType::Insert => {
                    let q = format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        table_name,
                        fields
                            .iter()
                            .map(|(k, _v)| format!("{}", k))
                            .collect::<Vec<String>>()
                            .join(", "),
                        fields
                            .iter()
                            .map(|(_k, v)| format!("'{}'", quote::quote!(#v).to_string()))
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                    println!("q: {q}");
                    q
                }
                DdlType::Create => {
                    let _domain_fields: VecDeque<StructDef> = VecDeque::new();
                    
                    let src = include_str!("../../src/model/entity.rs");
                    let raw = format!(r#"{src}"#);
                    let structs = parse_structs(&raw);
                    
                    let mut visited = HashSet::new();
                    let mut output = Vec::new();

                    for name in structs.keys() {
                        iter_structs(name, &structs, &mut visited, &mut output);
                    }

                    println!("create sql: {}", output.join("\n\n"));
                    output.join("\n\n")
                }
                DdlType::Alter => todo!(), //not yet implemented
                                           // format!(
                                           //     "ALTER TABLE {} ADD COLUMN {}",
                                           //     table_name
                                           //     fields
                                           //         .iter()
                                           //         .map(|(k, v)| format!("{} {}", k, v))
                                           //         .collect::<Vec<String>>()
                                           //         .join(", "
                                           // ),
                                           //TODO RENAME
                                           //TODO SET DATA TYPE
                                           //TODO CONSTRAINT
            },
        };
        instance
    }
}

fn parse_structs(input: &str) -> HashMap<String, StructDef> {
    let mut map = HashMap::new();
    let re_struct = Regex::new(r"(?s)(?:#\[.*?\]\s*)*(?:pub\s+)?struct\s+([A-Za-z0-9_]+)\s*\{(.*?)\}").unwrap();
    let re_field = Regex::new(r"(?m)^(?:\s*#\[.*\]\s*)*(?:\s*pub\s+)?\s*([A-Za-z0-9_]+)\s*:\s*([^,]+),").unwrap();
    for caps in re_struct.captures_iter(input) {
        let name = caps.get(1).unwrap().as_str().to_string();
        let body = caps.get(2).unwrap().as_str();

        let mut fields = Vec::new();
        for fc in re_field.captures_iter(body) {
            fields.push(Field {
                name: fc.get(1).unwrap().as_str().trim().to_string(),
                type_str: fc.get(2).unwrap().as_str().trim().to_string(),
            });
        }

        map.insert(name.clone(), StructDef { name, fields });
    }

    map
}

fn iter_structs(
    name: &str,
    structs: &HashMap<String, StructDef>,
    visited: &mut HashSet<String>,
    output: &mut Vec<String>,
) {
    let singular_name = util::singularize(name);
    if visited.contains(&singular_name) || visited.contains(name) {
        return;
    }
    visited.insert(singular_name.clone());

    let sdef = match structs.get(name) {
        Some(s) => s,
        None => return,
    };

    let mut sql = Vec::new();
    sql.push(format!("CREATE TABLE {} (", singular_name.to_lowercase()));
    sql.push("    id SERIAL PRIMARY KEY,".to_string());

    let mut post_sql = Vec::new();
    let mut constraints = Vec::new();

    for field in &sdef.fields {
        let (is_opt, inner) = util::strip_option(&field.type_str);

        if let Some(inner_vec) = util::is_vec(&inner) {
            // Vec<T> → child table
            let child_table = util::singularize(&field.name.to_lowercase());

            if visited.contains(&child_table) {
                continue;
            }
            let mut child = Vec::new();
            let singular_inner_vec = util::singularize(&inner_vec);
            if visited.contains(&singular_inner_vec) 
                || visited.contains(&singular_inner_vec.to_lowercase())
                || visited.contains(&inner_vec.to_lowercase())
                || visited.contains(&inner_vec) {
                    println!("");
            } else {
                visited.insert(child_table.clone());
            
                child.push(format!("CREATE TABLE {} (", child_table));
                child.push("    id SERIAL PRIMARY KEY,".to_string());
                child.push(format!("    {}_id INTEGER NOT NULL,", singular_name.to_lowercase()));
            }
            if let Some(sql_ty) = util::map_rust_type_to_sql(&inner_vec) {
                child.push(format!("    value {} NOT NULL,", sql_ty));
            } else if structs.contains_key(&singular_inner_vec) {
                iter_structs(&util::singularize(&inner_vec), structs, visited, output);
            
                let alt = format!(
                    "ALTER TABLE {} ADD COLUMN {}_id INTEGER NOT NULL;\n\
                     ALTER TABLE {} ADD CONSTRAINT fk_{}_{}_id FOREIGN KEY ({}_id) REFERENCES {}(id);",
                    singular_inner_vec.to_lowercase(),
                    singular_name.to_lowercase(),
                    singular_inner_vec.to_lowercase(),
                    singular_inner_vec.to_lowercase(),
                    singular_name.to_lowercase(),
                    singular_name.to_lowercase(),
                    singular_name.to_lowercase()
                );
                post_sql.push(alt);
                continue;
            } else {
                child.push("    value TEXT NOT NULL,".to_string());
            }

            // Use CONSTRAINT for foreign key
            child.push(format!(
                "    CONSTRAINT fk_{}_{}_id FOREIGN KEY ({}_id) REFERENCES {} (id)",
                child_table,
                singular_name.to_lowercase(),
                singular_name.to_lowercase(),
                singular_name.to_lowercase()
            ));
            child.push(");".to_string());

            post_sql.push(child.join("\n"));
            continue;
        }

        // primitive type
        if let Some(sql_ty) = util::map_rust_type_to_sql(&inner) {
            let null = if is_opt { "" } else { " NOT NULL" };
            sql.push(format!("    {} {}{},", field.name, sql_ty, null));
            continue;
        }

        // struct field → 1:1
        if structs.contains_key(&inner) {
            iter_structs(&util::singularize(&inner), structs, visited, output);

            let col = format!("{}_id", field.name);
            let null = if is_opt { "" } else { " NOT NULL" };

            sql.push(format!("    {} INTEGER{},", col, null));
            // Add constraint instead of direct FOREIGN KEY
            constraints.push(format!(
                "    CONSTRAINT fk_{}_{}_id FOREIGN KEY ({}) REFERENCES {}(id),",
                singular_name.to_lowercase(),
                col,
                col,
                inner.to_lowercase()
            ));
            continue;
        }

        // fallback: TEXT
        let null = if is_opt { "" } else { " NOT NULL" };
        sql.push(format!("    {} TEXT{},", field.name, null));
    }

    // Remove trailing comma from last column or constraint
    if let Some(last) = sql.last_mut() {
        *last = last.trim_end_matches(',').to_string();
    }

    // Add constraints (if any)
    for constraint in &constraints {
        sql.push(constraint.trim_end_matches(',').to_string());
    }

    sql.push(");".to_string());

    output.push(sql.join("\n"));

    for p in post_sql {
        output.push(p);
    }
}

pub enum DdlType {
    Insert,
    Create,
    Alter,
}

pub struct ResultRow {
    pub columns: HashMap<String, String>,
}

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
    fn query_schema(&mut self, table_name: &str) -> Result<HashMap<String, String>, DatabaseError>;
    fn execute_query(&mut self, query: ParameterizedQuery)
    -> Result<Vec<ResultRow>, DatabaseError>;
    fn execute_ddl(&mut self, ddl: Ddl);
    #[allow(dead_code)]
    fn close();
}

impl ActiveConnection for Connection {
    fn insert(&mut self, fields: &Vec<Field>, table_name: &str) -> Result<(), DatabaseError> {
        if util::is_sql(table_name) {
            for field in fields {
                println!("name: {}, sql_type: {}", field.name, field.type_str)
            }
            Err(DatabaseError {
                kind: InjectionAttempt,
            })
        } else {
            //TODO finish insert statement
            self.query = format!("INSERT INTO {table_name} VALUES (");
            let params = String::from("");
            let _ = self.client.query(&self.query, &[&params]);
            Ok(())
        }
    }

    fn query_schema(&mut self, table_name: &str) -> Result<HashMap<String, String>, DatabaseError> {
        let query = "select * 
        from information_schema.columns 
        where table_name = $1 and table_schema = 'public'";
        let result = self
            .client
            .query(query, &[&table_name])
            .map_err(|e| DatabaseError {
                kind: TableDoesNotExist(e),
            });
        let mut schema: HashMap<String, String> = HashMap::new();
        for i in result.unwrap() {
            schema.insert(
                i.try_get("column_name").map_err(|e| DatabaseError {
                    kind: UnknownType(e),
                })?,
                util::parse_sql_field(i.try_get("data_type").map_err(|e| DatabaseError {
                    kind: UnknownType(e),
                })?)?,
            );
        }
        Ok(schema)
    }

    fn execute_query(
        &mut self,
        query: ParameterizedQuery,
    ) -> Result<Vec<ResultRow>, DatabaseError> {
        let res = self
            .client
            .query(&query.sql_query, &[])
            .map_err(|e| DatabaseError {
                kind: InvalidQuery(e),
            })?;

        let rows_result: Result<Vec<ResultRow>, DatabaseError> = res
            .iter()
            .map(|row| -> Result<ResultRow, DatabaseError> {
                let mut columns: HashMap<String, String> = HashMap::new();
                for (i, column) in row.columns().iter().enumerate() {
                    let value: String = row.try_get(i).map_err(|e| DatabaseError {
                        kind: UnknownType(e),
                    })?;
                    columns.insert(column.name().to_string(), value);
                }
                Ok(ResultRow { columns })
            })
            .collect();

        let ret = rows_result?;
        Ok(ret)
    }

    fn execute_ddl(&mut self, ddl: Ddl) {
        let _ = self.client.query(&ddl.sql_statement, &[]);
    }

    fn close() {
        todo!()
        //"not yet implemented"
    }
}

pub fn connect() -> Result<Connection, DatabaseError> {
    // pub fn new() -> Result<Connection, DatabaseException> {
    match postgres_client(util::get_conf()?) {
        Ok(client) => Ok(Connection::new(client)),
        Err(e) => {
            // match e.into_source() {
            //     Ok(e2) => e2.
            // }
            // let s = e.kind().to_string();
            // println!("{}");
            Err(DatabaseError {
                kind: NotConnected(e),
            })
        } // Err(e) => Err("Something went wrong!".into()),
    }
}

fn postgres_client(conf: Conf) -> Result<Client, postgres::Error> {
    // fn post_gres_client() -> Result<Client, DatabaseException> {
    // fn persist(data_set: DataSet) -> Result<(), Error> {
    let _prot = "postgres";
    // let user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    // let pw = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    // let host = "localhost";
    // let db = "postgres";
    let user = conf.database_user;
    let pw = conf.database_password;
    let host = conf.database_host;
    let db = conf.database_name;

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

//TODO remove
fn iter_row(query_result: Vec<postgres::Row>) -> Result<(), postgres::Error> {
    Ok(for row in query_result {
        let id: i64 = row.try_get("id")?;
        let balance: f64 = row.try_get("balance")?;
        println!("Account {}: {}", id, balance);
    })
}
