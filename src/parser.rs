use crate::model::domain::Boat;
use crate::util;
use crate::model::http::Form;

use serde;
use serde_json::{self, Error};

pub fn to_model<T: serde::Deserialize>(payload: Form) -> T {
    let email = email("payload"); //FIXME payload
    validate(email);

    let t: Result<T, _> = serde_json::from_str(payload);
    t

    // match boat {
    //     Ok(t) => t,
    //     Err(e) => {
    //         println!("{}", e.io_error_kind().unwrap());
    //         // (ONLY IF): The kind reported by the underlying standard library I/O error, if this error was caused by a failure to read or write bytes on an I/O stream.
    //          //called `Option::unwrap()` on a `None` value
    //         match e.io_error_kind() {
    //         Some(std::io::ErrorKind::UnexpectedEof) => {String::from("")},
    //         _ => panic!("Can't parse data to boat, {:?}", e),
    //     }},
    // }
    // // if boat.unwrap().submitData == "1234"
    // // data
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
