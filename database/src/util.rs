use regex::Regex;
use std::collections::HashMap;
use crate::domain::{StructDef, Field};

use crate::DatabaseError;
use crate::DatabaseErrorKind;

pub struct Conf {
    pub database_user: String,
    pub database_password: String,
    pub database_host: String,
    pub database_name: String,
}

impl Conf {
    pub fn new() -> Self {
        Conf {
            database_user: String::new(),
            database_password: String::new(),
            database_host: String::new(),
            database_name: String::new(),
        }
    }
}

pub fn get_conf() -> Result<Conf, DatabaseError> {
    let file = std::fs::read_to_string("database/conf-secrets.txt")
        .map_err(|e| DatabaseErrorKind::ConfigUnavailable(e))
        .map_err(|kind| DatabaseError { kind })?;
    let mut conf = Conf::new();
    for line in file.lines() {
        let (k, v) = line.split_once('=').unwrap();
        if k == "DATABASE_USER" {
            conf.database_user = v.to_string();
        } else if k == "DATABASE_PASSWORD" {
            conf.database_password = v.to_string();
        } else if k == "DATABASE_HOST" {
            conf.database_host = v.to_string();
        } else if k == "DATABASE_NAME" {
            conf.database_name = v.to_string();
        }
    }
    Ok(conf)
}

#[allow(dead_code)]
pub fn not_null(data: Option<&str>) -> Option<&str> {
    match data {
        Some(data) => Some(data),
        None => None,
    }
}

pub fn is_sql(data: &str) -> bool {
    return data.contains(";")
        || data.to_lowercase().contains("select ")
        || data.to_lowercase().contains("drop ");
}


pub fn parse_sql_field(sql_field: &str) -> Result<String, DatabaseError> {
    //TODO impl more types
    let ty = sql_field.to_lowercase();
    println!("{ty}");
    if ty.starts_with("character varying") {
        Ok(String::from("String"))
    } else if ty.starts_with("int") {
        Ok(String::from("i32"))
    } else if ty.starts_with("bigint") {
        Ok(String::from("u64"))
    } else if ty.starts_with("double precision") {
        Ok(String::from("f32"))
    } else if ty.starts_with("boolean") {
        Ok(String::from("bool"))

    //TODO handle Vecs and other collections
    } else {
        Err(DatabaseError {
            kind: DatabaseErrorKind::UnknownInputType(ty),
        })
    }
}

pub fn parse_structs(input: &str) -> HashMap<String, StructDef> {
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

pub fn map_rust_type_to_sql(ty: &str) -> Option<&'static str> {
    match ty {
        "u32" | "i32" | "u16" | "i16" | "u8" | "i8" => Some("INTEGER"),
        "u64" | "i64" => Some("BIGINT"),
        "String" | "&str" => Some("TEXT"),
        "f32" | "f64" => Some("DOUBLE PRECISION"),
        "bool" => Some("BOOLEAN"),
        _ => None,
    }
}

pub fn strip_option(ty: &str) -> (bool, String) {
    let opt_re = Regex::new(r"^Option\s*<\s*(.+)\s*>$").unwrap();
    if let Some(c) = opt_re.captures(ty) {
        (true, c[1].trim().to_string())
    } else {
        (false, ty.to_string())
    }
}

pub fn is_vec(ty: &str) -> Option<String> {
    let vec_re = Regex::new(r"^Vec\s*<\s*(.+)\s*>$").unwrap();
    vec_re.captures(ty).map(|c| c[1].trim().to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_null_test1() {
        assert_eq!(not_null(Some("")), Some(""));
    }

    #[test]
    fn is_sql_test1() {
        assert_eq!(is_sql("DROP DATABASE"), true);
    }

    #[test]
    fn is_sql_test2() {
        assert_eq!(is_sql("customer;select * from account;"), true);
    }

    #[test]
    fn is_sql_test3() {
        assert_eq!(is_sql("customer"), false);
    }
}
