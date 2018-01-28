#[macro_use] extern crate compile_validate;

validate_json!("good.json");

fn main() {
    println!("It runs");
}
