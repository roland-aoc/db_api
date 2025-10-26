pub fn check_len(data: &str) -> usize {
    data.chars().count()
}

#[allow(dead_code)]
pub fn not_null(data: Option<&str>) -> Option<&str> {
    match data {
        Some(data) => Some(data),
        None => None
    }
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
}
