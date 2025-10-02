use crate::domain::Boat;
use crate::util;
use serde_json;

pub fn payload_to_model(data: &str) -> String {
    let email = email(data);
    validate(email);

    // let boat= Boat {id: 1, name: String::from("MyBoat")};
    let boat: Result<Boat, _> = serde_json::from_str(data);
    match boat {
        Ok(t) => t.submitSource,
        Err(e) => panic!("Can't parse data to boat, {:?}", e)
    }
    // if boat.unwrap().submitData == "1234"
    // data
}

fn email(data: &str) -> &str {
    "writerolandkurmann@gmail.com"
}

fn validate(input: &str) {
    print!("{input}");
    let len = util::check_len("");
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
