use crate::service;
use crate::model::http::Payload;

use serde_json::{self};
use warp::{Filter, http::Response};
use std::collections::HashMap;
use std::net::Ipv4Addr;

// pub async fn post(request_body: String) -> Result<(), Box<dyn std::error::Error>> {
pub async fn post() {
    // if let Some(pass_on) = data {
    //     Some(parser::payload_to_model(&pass_on))
    // } else {Some(String::from(""))}

    let example1 = warp::post()
        // .and(warp::path("api"))
        // .and(warp::path("db_api"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("name") {
            Some(name) => {
                let _ = parse(name);
                Response::builder().status(200).body(format!("Hello, {}. This HTTP triggered function executed successfully.", name))
            },
            None => Response::builder().status(500).body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await;
}
    
fn parse(request_body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let payload: Result<Payload, _> = serde_json::from_str(&request_body);
    // println!("Payload result: {:?}", payload.as_ref());
    match payload {
        Ok(p) => service::handle(p),
        Err(e) => Err(Box::from(e))
    }
    }