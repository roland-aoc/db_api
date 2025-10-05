use crate::service;
use crate::model::http::Form;
use crate::model::http::Rumpfmaterial;

pub fn post(data: Option<&str>) {
    // if let Some(pass_on) = data {
    //     Some(parser::payload_to_model(&pass_on))
    // } else {Some(String::from(""))}
    let form = Form {
        q6_email6: String::from(""), 
        rumpfmaterial: Rumpfmaterial::ALUMINIUM, 
        rigg_typ: String::from("")
    };
    service::persist(form); //FIXME
}