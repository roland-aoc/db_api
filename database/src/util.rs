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
    if sql_field.to_lowercase().starts_with("varchar") {
        Ok(String::from("String"))
    } else if sql_field.to_lowercase().starts_with("int") {
        Ok(String::from("u32"))
    } else if sql_field.to_lowercase().starts_with("float") {
        Ok(String::from("f32"))
    } else if sql_field.to_lowercase().starts_with("boolean") {
        Ok(String::from("bool"))
    } else {
        Err(DatabaseError {
            kind: DatabaseErrorKind::UnknownInputType,
        })
    }
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
