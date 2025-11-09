use crate::DatabaseError;
use crate::DatabaseErrorKind;

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
    if sql_field.to_lowercase().starts_with("varchar") {
        Ok(String::from("String"))
    } else if sql_field.to_lowercase().starts_with("int") {
        Ok(String::from("u32"))
    } else if sql_field.to_lowercase().starts_with("boolean") {
        Ok(String::from("bool"))
    } else if sql_field.to_lowercase().starts_with("float") {
        Ok(String::from("f32"))
    } else {
        Err(DatabaseError { kind: DatabaseErrorKind::UnknownInputType, })
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
