use crate::domain::Boat;
pub fn payload_to_model(data: &str) -> &str {
    let boat= Boat {id: 1, name: String::from("MyBoat")};
    email(data)
}

fn email(data: &str) -> &str {
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
