use crate::model::{http, mapper};
use crate::repository;
use crate::util;

pub fn handle(payload: http::Payload) -> Result<(), Box<dyn std::error::Error>> {
    // let model: Option<domain::DataSet> = Some(mapper::to_model::<domain::DataSet>(payload));
    let model = mapper::to_data_set(payload);
    validate(model.length);

    // repository::persist(model.expect("Model is empty!"))
    repository::create(model)
}

fn validate(input: u8) {
    println!("Validate email, which is {input}");
    let _len = util::check_len("");
}
#[allow(dead_code)]
fn email(_data: &str) -> &str {
    "writerolandkurmann@gmail.com"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_test1() {
        assert_eq!(
            email(
                "{ \"id\": null, \"name\": \"Sea Breeze\", \"boat_type\": \"Sail\", \"length\": 12.5, \"year\": 2018 }"
            ),
            "writerolandkurmann@gmail.com"
        );
    }
}