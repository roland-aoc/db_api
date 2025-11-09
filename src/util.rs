pub fn check_len(data: &str) -> usize {
    data.chars().count()
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
}
