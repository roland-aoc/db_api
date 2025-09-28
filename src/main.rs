mod util;
mod parser;
mod domain;

fn main() {
    let input = "";
    parser::payload_to_model(input);
    validate(input);
}

fn validate(input: &str) {
    print!("{input}");
    let len = util::check_len("");
}
