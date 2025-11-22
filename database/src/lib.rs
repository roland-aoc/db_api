pub mod util;
pub mod domain;
use domain::{StructDef};

use postgres::{Client, NoTls};
use std::collections::{HashMap, HashSet, VecDeque};
use syn;

use crate::util::Conf;

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

pub struct Field {
    pub name: String,
    pub sql_type: String,
}

impl Field {
    #[allow(dead_code)]
    fn new(name: String, sql_type: String) -> Self {
        Self { name, sql_type }
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
                    let structs = util::parse_structs(&raw);
                    
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

fn iter_structs(
    name: &str,
    structs: &HashMap<String, StructDef>,
    visited: &mut HashSet<String>,
    output: &mut Vec<String>,
) {
    if visited.contains(name) {
        return;
    }
    visited.insert(name.to_string());

    let sdef = match structs.get(name) {
        Some(s) => s,
        None => return,
    };

    let mut sql = Vec::new();
    sql.push(format!("CREATE TABLE {} (", sdef.name.to_lowercase()));
    sql.push("    id SERIAL PRIMARY KEY,".to_string());

    let mut post_sql = Vec::new();

    for field in &sdef.fields {
        let (is_opt, inner) = util::strip_option(&field.type_str);

        if let Some(inner_vec) = util::is_vec(&inner) {
            // Vec<T> → child table
            let child_table = format!("{}_{}", name.to_lowercase(), field.name.to_lowercase());

            let mut child = Vec::new();
            child.push(format!("CREATE TABLE {} (", child_table));
            child.push("    id SERIAL PRIMARY KEY,".to_string());
            child.push(format!("    {}_id INTEGER NOT NULL,", name.to_lowercase()));

            if let Some(sql_ty) = util::map_rust_type_to_sql(&inner_vec) {
                child.push(format!("    value {} NOT NULL,", sql_ty));
            } else if structs.contains_key(&inner_vec) {
                // Vec<Struct> → generate struct and FK inside that struct table
                iter_structs(&inner_vec, structs, visited, output);

                // Add FK later via ALTER
                let alt = format!(
                    "ALTER TABLE {} ADD COLUMN {}_id INTEGER NOT NULL;\n\
                     ALTER TABLE {} ADD CONSTRAINT fk_{}_{}_id FOREIGN KEY ({}_id) REFERENCES {}(id);",
                    inner_vec.to_lowercase(),
                    name.to_lowercase(),
                    inner_vec.to_lowercase(),
                    inner_vec.to_lowercase(),
                    name.to_lowercase(),
                    name.to_lowercase(),
                    name.to_lowercase()
                );
                post_sql.push(alt);
            } else {
                child.push("    value TEXT NOT NULL,".to_string());
            }

            child.push(format!(
                "    FOREIGN KEY ({}_id) REFERENCES {}(id)",
                name.to_lowercase(),
                name.to_lowercase()
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
            iter_structs(&inner, structs, visited, output);

            let col = format!("{}_id", field.name);
            let null = if is_opt { "" } else { " NOT NULL" };

            sql.push(format!("    {} INTEGER{},", col, null));
            sql.push(format!(
                "    FOREIGN KEY ({}) REFERENCES {}(id),",
                col,
                inner.to_lowercase()
            ));
            continue;
        }

        // fallback: TEXT
        let null = if is_opt { "" } else { " NOT NULL" };
        sql.push(format!("    {} TEXT{},", field.name, null));
    }

    // Clean last comma
    if let Some(last) = sql.last_mut() {
        *last = last.trim_end_matches(',').to_string();
    }

    sql.push(");".to_string());

    output.push(sql.join("\n"));

    for p in post_sql {
        output.push(p);
    }
}

//TODO remove
fn _iter_fields_old(
    mut depth: u8,
    table_name: &str,
    fields: &HashMap<&syn::Ident, &syn::Type>,
    outer_sql_fields: &mut HashMap<u8, Vec<String>>,
    outer_constraints: &mut HashMap<u8, Vec<String>>,
) -> String {
    let mut final_create_statement = String::new();
    for (k, v) in fields.iter() {
        let parsed = util::parse_rust_field(v)
            .map_err(|e| panic!("{:?}", e))
            .unwrap();
        if parsed.starts_with("new 1:1 relation") {
            let ty = parsed.split(",").nth(1);
            let ast: syn::DeriveInput = syn::parse(ty.unwrap().parse().unwrap()).unwrap();
            if let syn::Data::Struct(d) = ast.data {
                let field_map: HashMap<syn::Ident, syn::Type> = d
                    .fields
                    .clone()
                    .into_iter()
                    .map(|f| (f.ident.unwrap(), f.ty))
                    .collect();
                let borrowed_field_map: HashMap<&syn::Ident, &syn::Type> =
                    field_map.iter().map(|(k, v)| (k, v)).collect();
                depth += 1;
                //TODO is this correct?
                outer_sql_fields.insert(depth.clone(), Vec::new());
                outer_constraints.insert(depth.clone(), Vec::new());
                _iter_fields_old(
                    depth,
                    k.to_string().as_ref(),
                    &borrowed_field_map,
                    outer_sql_fields,
                    outer_constraints,
                );
                depth -= 1;
            }
            let field_type = util::parse_rust_field(v)
                .map_err(|e| panic!("{:?}", e))
                .unwrap();

            let field_name = field_type.split(",").nth(0).unwrap();
            let sub_type = parsed.split(",").nth(1).unwrap();
            let relations = format!(
                "CREATE TABLE {} (
                    id SERIAL PRIMARY KEY,
                    {} {};
                );\n\n",
                field_name, field_name, sub_type,
            );
            final_create_statement.push_str(&relations);
            let constraint = format!(
                "CONSTRAINT fk_{} 
                    FOREIGN KEY (id) 
                    REFERENCES {} ({}_id) 
                    ON DELETE SET NULL\n\n",
                field_name, field_name, field_name
            )
            .to_string();
            
            //TODO
            // child table, i.e. dim, references the fact table key
            //constraint is on the 1 table, the dim
            //i.e. fact table should be created first
            //for 1:1 chhild table reference id is marked as UNIQUE
            
            let mut vec1 = Vec::new();
            vec1.push(format!("{}_id bigint", k.to_string()));
            outer_sql_fields.insert(depth.clone(), vec1);
            let mut vec2 = Vec::new();
            vec2.push(constraint);
            outer_constraints.insert(depth.clone(), vec2);
            continue;
        } else if parsed.starts_with("new 1:many relation") {
            // depth += 1;
            let sub_type = parsed.split(",").nth(1).unwrap();
            let relation = format!(
                // "CREATE TABLE {} (
                    "id SERIAL PRIMARY KEY,
                    {}_id bigint,
                    {} {}\n"
                // );\n\n",
                ,
                // k.to_string(),
                table_name,
                k.to_string(),
                sub_type
            );
            let constraint = format!(
                "CONSTRAINT fk_{} 
                    FOREIGN KEY (id) 
                    REFERENCES {} ({}_id) 
                    ON DELETE SET NULL",
                k.to_string(),
                table_name,
                k.to_string()
            )
            .to_string();
            final_create_statement.push_str(
                format!(
                    "CREATE TABLE {} ({}, {});\n\n",
                    k.to_string(),
                    relation,
                    constraint
                )
                .to_string()
                .as_ref(),
            );
            // final_create_statement.push_str(&relation);
            // depth -= 1;
            continue;
        } else {
            outer_sql_fields
                .get_mut(&depth)
                .expect("the depth of outer_sql_fields should be available")
                .push(format!("{} {}", k, parsed));
            outer_constraints.insert(depth.clone(), Vec::new());
        }
    }
    final_create_statement.push_str(&format!(
        "CREATE TABLE {} ({}, {});\n\n",
        table_name,
        outer_sql_fields
            .get(&depth)
            .expect("the depth of outer_sql_fields should be available")
            .join(", "),
        outer_constraints
            .get(&depth)
            .expect("the depth of outer_constraints should be available")
            .join(", ")
    ));
    final_create_statement
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
                println!("name: {}, sql_type: {}", field.name, field.sql_type)
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

// fn derive_constraints_and_yield_relations(
//     fields: &HashMap<&syn::Ident, &syn::Type>,
//     parent_field_name: Option<&str>,
//     final_create_statement: &mut String,
// ) -> Option<(String, String)> {
//     for f in fields {
//         let field_type = util::parse_rust_field(f.1)
//         .map_err(|e| panic!("{:?}", e))
//             .unwrap();
//         if field_type.starts_with("new 1:many relation") {
//             let sub_type = field_type.split(",").nth(1).unwrap();
//             let relations = format!(
//                 "CREATE TABLE {} (
//                     id SERIAL PRIMARY KEY,
//                     {} {},
//                     {}_id bigint
//                 )",
//                 sub_type,
//                 fields,
//                 parent_field_name.unwrap(),
//             );
//             let constraint = format!(
//                 "CONSTRAINT fk_{}
//                     FOREIGN KEY (id)
//                     REFERENCES {} ({}_id)
//                     ON DELETE SET NULL",
//                     sub_type,
//                     sub_type, sub_type
//                 )
//                 .to_string();
//             final_create_statement.push_str(format!("", ""));
//             Some((relations, constraint));
//         } else if field_type.starts_with("new 1:1 relation") {
//             let field_name = field_type.split(",").nth(0).unwrap();
//             let sub_type = field_type.split(",").nth(1).unwrap();
//             let relations = format!(
//                 "CREATE TABLE {} (
//                     id SERIAL PRIMARY KEY,
//                     {} {}
//                 )",
//                 field_name,
//                 field_name, sub_type,
//             );
//             let constraint = format!(
//                 "CONSTRAINT fk_{}
//                     FOREIGN KEY (id)
//                     REFERENCES {} ({}_id)
//                     ON DELETE CASCADE",
//                 field_name,
//                 field_name, field_name
//             )
//             .to_string();
//             Some((relations, constraint));
//         }
//     }
//     final_create_statement.push_str(&format!(
//                             "CREATE TABLE {} ({}, {});",
//                             parent_field_name,
//                             ready_sql_fields.join(", "),
//                             creates_and_constraints.1
//                         ));
//     CREATE TABLE users (
//     user_id SERIAL PRIMARY KEY,
//     username VARCHAR(50) NOT NULL
// );

// CREATE TABLE products (
//     product_id SERIAL PRIMARY KEY,
//     product_name VARCHAR(100) NOT NULL,
//     price NUMERIC(10, 2) NOT NULL
// );

//     CREATE TABLE orders (
//     order_id SERIAL PRIMARY KEY,
//     user_id INT NOT NULL,
//     product_id INT NOT NULL,
//     quantity INT NOT NULL,
//     order_date TIMESTAMP NOT NULL,

//     -- First Foreign Key (referencing the users table)
//     CONSTRAINT fk_user
//         FOREIGN KEY (user_id)
//         REFERENCES users (user_id)
//         ON DELETE CASCADE,

//     -- Second Foreign Key (referencing the products table)
//     CONSTRAINT fk_product
//         FOREIGN KEY (product_id)
//         REFERENCES products (product_id)
//         ON DELETE RESTRICT
