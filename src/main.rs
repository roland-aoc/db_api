use server;

fn main() {
    let input = "main: Hello, World!";
    server::receive(Some(input));
    println!("{input}")
    // parser::payload_to_model(input);
}

