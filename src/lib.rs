mod util;
mod parser;
mod domain;

pub fn receive(data: Option<&str>) -> Option<&str>{
    if let Some(pass_on) = data {
        parser::payload_to_model(&pass_on);
    }
    data
}