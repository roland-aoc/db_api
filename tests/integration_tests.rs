use server;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_receives_raw_request() {
        let data = std::fs::read_to_string("tests/fixtures/raw_request.json")
            .expect("could not read fixture");
        assert_eq!(server::receive(Some(&data)).unwrap(), "form");
    }

    #[test]
    fn it_empty_raw_request() {
        let data = std::fs::read_to_string("tests/fixtures/empty_raw_request.json")
            .expect("could not read fixture");
        assert_eq!(server::receive(Some(&data)).unwrap(), "");
    }
}
