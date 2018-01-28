#[macro_use] extern crate compile_validate;

validate_json!("bad.json");

fn main() {
    println!("It runs");
}
