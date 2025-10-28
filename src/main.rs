use db_api;
use serde_json::json;
use std::io::{self, Read, Write};

// use tiny_http::{Server, Response};

#[tokio::main]
// async fn main() -> std::io::Result<()> {
async fn main() {
    dotenvy::dotenv().ok();


    // let input = "main: Hello, World!";
    // println!("{input}")
    // // parser::payload_to_model(input);
    db_api::receive().await;
}

// fn tiny_http() {
//     let server = Server::http("localhost:8089").unwrap();
//     for request in server.incoming_requests() {
//         let response = Response::from_string("Hello from Rust via HTTP forwarding!");
//         request.respond(response).unwrap();
//     }
// }
#[allow(dead_code)]
fn stdin_stdout_conf() {
    let mut input: String = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read stdin: {}", err);
        return;
    }

    // Parse the incoming request
    let request: serde_json::Value = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return;
        }
    };

    let _name = request["Data"]["name"].as_str().unwrap_or("world");

    // Prepare the response
    let response = json!({
        "statusCode": 200,
        "body": format!("Hello, {} from Rust!", "HARDCODED")
    });

    // Write response to stdout
    let output = response.to_string();

    // Use stdout directly + flush (important!)
    let mut stdout = io::stdout();
    stdout.write_all(output.as_bytes()).unwrap();
    stdout.flush().unwrap();
}
