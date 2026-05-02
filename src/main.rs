use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use encrypt::{caesar_cipher, vigenere_cipher, columnar_cipher, xor_cipher, railfence_cipher};
use clap::{Command, Arg, ArgAction};

pub fn read_file(mut file_name: String) -> String {
    loop {
        match File::open(&file_name) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Failed to read file.");
                return contents;
            }
            Err(_) => {
                file_name = prompt("File Not Found! Please enter new File Path: ");
            }
        }
    }
}

fn prompt(label: &str) -> String {
    print!("{}", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn interactive_mode() {
    let input_mode = prompt("Do you want to read from a (F)ile or type (T)ext manually? ");
    let text = if input_mode.trim().to_uppercase().starts_with('F') {
        let file_path = prompt("File Path: ");
        read_file(file_path)
    } else {
        prompt("Enter your text: ")
    };
    println!("Ciphers: 1. Caesar Cipher, 2. Vigenere Cipher, 3. Columnar Cipher, 4. XOR Cipher, 5. Rail Fence Cipher");
    let choice = prompt("Select a cipher (1-5): ");
    let mode = prompt("Do you want to (E)ncrypt or (D)ecrypt? ");
    let decrypt = mode.trim().to_uppercase().starts_with('D');
    let result = match choice.as_str() {
        "1" => {
            loop {
                let key_str = prompt("Enter shift amount: ");
                match caesar_cipher(text.clone(), key_str, decrypt) {
                    Ok(result) => break result,
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
        "2" => {
            loop {
                let key = prompt("Enter text key: ");
                match vigenere_cipher(text.clone(), key, decrypt) {
                    Ok(result) => break result,
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
        "3" => {
            loop {
                let key = prompt("Enter text key: ");
                match columnar_cipher(text.clone(), key, decrypt) {
                    Ok(result) => break result,
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
        "4" => {
            loop {
                let key = prompt("Enter text key: ");
                match xor_cipher(text.clone(), key, decrypt) {
                    Ok(result) => break result,
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
        "5" => {
            loop {
                let key_str = prompt("Enter height of fence: ");
                match railfence_cipher(text.clone(), key_str, decrypt) {
                    Ok(result) => break result,
                    Err(e) => println!("Error: {}", e),
                }
            }
        },
        _ => {
            println!("Not a valid cipher :(");
            return;
        }
    };
    println!("{}", result);
}

fn cli_mode(matches: clap::ArgMatches) {
    let text = if let Some(t) = matches.get_one::<String>("text") {
        t.clone()
    } else if let Some(f) = matches.get_one::<String>("file") {
        read_file(f.clone())
    } else {
        unreachable!("Need either text or file");
    };

    let cipher = matches.get_one::<String>("cipher").unwrap();
    let key = matches.get_one::<String>("key").unwrap().clone();
    let decrypt = matches.get_flag("decrypt");

    let result = match cipher.as_str() {
        "caesar" => caesar_cipher(text, key, decrypt),
        "vigenere" => vigenere_cipher(text, key, decrypt),
        "columnar" => columnar_cipher(text, key, decrypt),
        "xor" => xor_cipher(text, key, decrypt),
        "railfence" => {
            let height: i32 = key.parse().unwrap_or(3);
            railfence_cipher(text, height, decrypt)
        }
        _ => unreachable!("Invalid cipher"),
    };
    println!("{}", result);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        interactive_mode();
    } else {
        let cmd = Command::new("encrypt")
            .about("A multi-cipher encryption tool")
            .arg(
                Arg::new("text")
                    .short('t')
                    .long("text")
                    .help("The text to encrypt/decrypt")
                    .conflicts_with("file")
                    .required_unless_present("file")
            )
            .arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .help("Path to the file containing text")
                    .conflicts_with("text")
                    .required_unless_present("text")
            )
            .arg(
                Arg::new("cipher")
                    .short('c')
                    .long("cipher")
                    .help("The cipher to use")
                    .value_parser(["caesar", "vigenere", "columnar", "xor", "railfence"])
                    .required(true)
            )
            .arg(
                Arg::new("key")
                    .short('k')
                    .long("key")
                    .help("Key or shift amount for the cipher")
                    .required(true)
            )
            .arg(
                Arg::new("decrypt")
                    .short('d')
                    .long("decrypt")
                    .help("Decrypt instead of encrypt")
                    .action(ArgAction::SetTrue)
            );

        let matches = cmd.get_matches();
        cli_mode(matches);
    }
}
