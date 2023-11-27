// Question mark operator
// ?
use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?; // return Error
    return Ok(parsed_number);
}
fn main() {
    println!("\nEXEC01");
    // EXEC01 ->

    for item in vec!["Seven", "8", "9.0", "nice", "6060"] {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
}
