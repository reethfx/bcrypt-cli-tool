use bcrypt::{hash, verify, DEFAULT_COST};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("bcrypt-cli")
        .version("1.0")
        .author("Reeth <reeth@b3rsrk.dev>")
        .about("Encrypt, compare a string using bcrypt")
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt a string using bcrypt")
                .arg(Arg::new("string")
                    .help("The string to encrypt")
                    .required(true))
                .arg(Arg::new("salt")
                    .help("Number of salt rounds for bcrypt")
                    .required(true)),
        )
        .subcommand(
            Command::new("compare")
                .about("Compare a string with a bcrypt hash")
                .arg(Arg::new("string")
                    .help("The string to compare")
                    .required(true))
                .arg(Arg::new("hash")
                    .help("The bcrypt hash to compare with")
                    .required(true)),
        )
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("encrypt") {
        let input_string = sub_matches.get_one::<String>("string").expect("String required");
        let salt_str = sub_matches.get_one::<String>("salt").expect("Salt required");

        let salt: u32 = salt_str.parse().expect("Salt should be a number");

        match hash(input_string, salt) {
            Ok(hashed) => {
                println!("Encrypted string: {}", hashed);
            }
            Err(e) => {
                eprintln!("Error encrypting: {}", e);
            }
        }
    }

    if let Some(sub_matches) = matches.subcommand_matches("compare") {
        let input_string = sub_matches.get_one::<String>("string").expect("String required");
        let hash_to_compare = sub_matches.get_one::<String>("hash").expect("Hash required");

        // Verificar si el string coincide con el hash
        match verify(input_string, hash_to_compare) {
            Ok(true) => println!("The provided string matches the hash."),
            Ok(false) => println!("The provided string does NOT match the hash."),
            Err(e) => eprintln!("Error comparing the string and hash: {}", e),
        }
    }
}
