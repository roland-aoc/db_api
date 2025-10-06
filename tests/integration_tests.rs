use server;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_receives_raw_request() {
        let data = std::fs::read_to_string("tests/fixtures/raw_request.json")
            .expect("could not read fixture");
        assert!(server::receive(data).is_ok());
    }

    #[test]
    fn it_empty_raw_request() {
        let data = std::fs::read_to_string("tests/fixtures/empty_raw_request.json")
            .expect("could not read fixture");
        assert!(server::receive(data).is_ok());
    }
    
    #[test]
    fn it_wip_request() {
        let data = std::fs::read_to_string("tests/fixtures/wip_request.json")
        .expect("could not read fixture");
        assert!(server::receive(data).is_ok());
    }
}
