use crate::domain::Boat;
use crate::util;

pub fn payload_to_model(data: &str) -> &str {
    let email = email(data);
    validate(email);
    
    let boat= Boat {id: 1, name: String::from("MyBoat")};
    data
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
