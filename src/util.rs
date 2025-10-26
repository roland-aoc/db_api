pub fn check_len(data: &str) -> usize {
    data.chars().count()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_len_test1() {
        assert_eq!(check_len("cars"), 4);
    }

    #[test]
    fn check_len_test2() {
        assert_eq!(check_len("me"), 2);
    }

    #[test]
    fn check_len_test3() {
        assert_eq!(check_len("Dampskibfartselskab"), 19);
    }

    #[test]
    fn check_len_test4() {
        assert_eq!(check_len("Car 😀"), 5);
    }

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
