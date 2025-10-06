use crate::model::domain::Boat;
use crate::model::http::Payload;
use crate::util;

use serde;

pub fn to_model<T>(payload: Payload) -> T {
    let map = std::collections::HashMap::from([
        ("q6_email6", "given_name"),
        ("rumpfmaterial", "surname"),
        ("rigg_typ", "years_old"),
    ]);

    //or macro to parse file ,

    let email = email("payload");
    validate(email);

    // let t: Result<T, _> = serde_json::from_str(payload);
    let t: Option<T> = None;
    t.unwrap() //FIXME

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
