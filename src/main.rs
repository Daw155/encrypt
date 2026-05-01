use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use encrypt::{caesar_cipher, vigenere_cipher, columnar_cipher, xor_cipher, railfence_cipher};

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

fn main() {
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
            let key_str = prompt("Enter shift amount: ");
            caesar_cipher(text, key_str, decrypt)
        },
        "2" => {
            let key = prompt("Enter text key: ");
            vigenere_cipher(text, key, decrypt)
        },
        "3" => {
            let key = prompt("Enter text key: ");
            columnar_cipher(text, key, decrypt)
        },
        "4" => {
            let key = prompt("Enter text key: ");
            xor_cipher(text, key, decrypt)
        },
        "5" => {
            let key_str = prompt("Enter height of fence: ");
            let height: i32 = key_str.parse().unwrap_or(3);
            railfence_cipher(text, height, decrypt)
        },
        _ => {
            println!("Not a valid cipher :(");
            return;
        }
    };
    println!("{}", result);
}
