use bcrypt::{hash, DEFAULT_COST};
use clap::{Arg, Command};
use std::io::{self, Write};

fn main() {
    let matches = Command::new("bcrypt-cli")
        .version("1.0")
        .author("Reeth <reeth@b3rsrk.dev>")
        .about("bcrypt cyphering on CLI")
        .arg(
            Arg::new("string")
                .help("The string to encrypt")
                .required(false),  // No es requerido, lo pedimos interactivamente si no se proporciona
        )
        .arg(
            Arg::new("salt")
                .help("Number of salt rounds for bcrypt (default: 12)")
                .required(false),
        )
        .get_matches();

    let input_string = if let Some(s) = matches.get_one::<String>("string") {
        s.to_string()
    } else {
        print!("Input the string to encrypt: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input error");
        input.trim().to_string()
    };

    let salt: u32 = if let Some(s) = matches.get_one::<String>("salt") {
        s.parse().unwrap_or(DEFAULT_COST)
    } else {
        print!("Input the salt number (Enter to use default 12): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input error");
        input.trim().parse().unwrap_or(DEFAULT_COST)
    };

    match hash(input_string, salt) {
        Ok(hashed) => {
            println!("Encrypted string: {}", hashed);
        }
        Err(e) => {
            eprintln!("Error encrypting: {}", e);
        }
    }
}
