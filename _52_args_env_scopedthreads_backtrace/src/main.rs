use std::env;

enum Letters {
    Capitalize,
    Lowercase,
    Nothing,
}
fn main() {
    println!("\nEXEC01");
    let args = env::args();
    println!("{args:?}");

    println!("\nEXEC02");
    let mut changes = Letters::Nothing;
    let input: Vec<String> = env::args().collect();

    if input.len() > 1 {
        match input[1].as_str() {
            "capital" => {
                changes = Letters::Capitalize;
            }
            "lowercase" => {
                changes = Letters::Lowercase;
            }
            _ => {}
        }
    }

    for word in env::args().skip(2) {
        match changes{
            Letters::Capitalize => println!("{}", word.to_uppercase()),
            Letters::Lowercase => println!("{}", word.to_lowercase()),
            Letters::Nothing => {},
        }
    }
    
    // cargo run capital ~~~~, cargo run lowercase ~~~

    println!("\nEXEC03");
    for v in std::env::vars(){
        println!("{v:?}");
    }
}
