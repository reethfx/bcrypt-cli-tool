use bcrypt::{hash, DEFAULT_COST};
use std::io::{self, Write};

fn main() {
    print!("Welcome to reeth's bcrpyt CLI Tool");
    io::stdout().flush().unwrap();
    print!("Enter the chain you want to cypher: ");
    io::stdout().flush().unwrap();
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Input error");
    let input_string = input_string.trim();

    print!("Input the Bcrypt salt number (12 by default): ");
    io::stdout().flush().unwrap();
    let mut salt_input = String::new();
    io::stdin()
        .read_line(&mut salt_input)
        .expect("Input error");

    let salt: u32 = match salt_input.trim().parse() {
        Ok(s) => s,
        Err(_) => DEFAULT_COST,
    };

    match hash(input_string, salt) {
        Ok(hashed) => {
            println!("Ciphered chain: {}", hashed);
        }
        Err(e) => {
            eprintln!("Error on the chain cypher: {}", e);
        }
    }
}
