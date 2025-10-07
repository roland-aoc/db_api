use crate::model::domain::{Boat, DataSet};
use crate::model::http::Payload;

use serde;

impl From<Payload> for DataSet {
    fn from(payload: Payload) -> Self {
        Self {
            email: payload.q6_email6,
        }
    }
}

// pub fn to_model<T>(payload: Payload) ->Result<(), Box<dyn std::error::Error>> {
pub fn to_data_set(payload: Payload) -> DataSet {
    return payload.into();

    // let t: Option<T> = None;
    // t.unwrap() //FIXME

    // let fields = std::collections::HashMap::from([
    //     ("q6_email6", "email"),
    //     ("q7_rumpfmaterial", "rumpfmaterial"),
    //     ("q8_rigg_typ", "rigg_typ"),
    // ]);
    // for (from, to) in fields {}

    //or macro to parse file ,
    // let t: Result<T, _> = serde_json::from_str(payload);

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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_model_test1() {
        let input = Payload { 
            q6_email6: String::from("writerolandkurmann@gmail.com"), 
            rumpfmaterial: crate::model::http::Rumpfmaterial::ALUMINIUM, 
            rigg_typ: String::from("") 
        };
        // let model = to_model::<DataSet>(input);
        // assert_eq!(model, DataSet {email: String::from("writerolandkurmann@gmail.com")});
    }
}