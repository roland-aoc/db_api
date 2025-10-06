use crate::service;
use crate::model::http::Payload;
use crate::model::http::Rumpfmaterial;

use serde_json::{self, Error};

pub fn post(request_body: String) -> Result<(), Box<dyn std::error::Error>> {
    // if let Some(pass_on) = data {
    //     Some(parser::payload_to_model(&pass_on))
    // } else {Some(String::from(""))}

    // let form = Payload {
    //     q6_email6: String::from(""), 
    //     rumpfmaterial: Rumpfmaterial::ALUMINIUM, 
    //     rigg_typ: String::from("")
    // };

    let payload: Result<Payload, _> = serde_json::from_str(&request_body);
    // println!("Payload result: {:?}", payload.as_ref());
    match payload {
        Ok(p) => service::handle(p),
        Err(e) => panic!("Error while parsing payload, {e:}")
    }
}