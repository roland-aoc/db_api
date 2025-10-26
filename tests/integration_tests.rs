use db_api;

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_receives_raw_request() {
        dotenvy::dotenv().ok();
        let data = std::fs::read_to_string("tests/fixtures/raw_request.json")
            .expect("could not read fixture");
        assert!(db_api::receive(data).is_ok());
    }

    // #[ignore]
    #[test]
    fn it_empty_raw_request() {
        dotenvy::dotenv().ok();
        let data = std::fs::read_to_string("tests/fixtures/empty_raw_request.json")
            .expect("could not read fixture");
        assert!(db_api::receive(data).is_err());
    }

    // #[ignore]
    #[test]
    fn it_wip_request() {
        dotenvy::dotenv().ok();
        let data = std::fs::read_to_string("tests/fixtures/wip_request.json")
            .expect("could not read fixture");
        assert!(db_api::receive(data).is_ok());
    }
}
