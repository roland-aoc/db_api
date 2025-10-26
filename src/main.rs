use server;

fn main() {
    let input = "main: Hello, World!";
    let _ = server::receive(String::from(""));
    println!("{input}")
    // parser::payload_to_model(input);
}
