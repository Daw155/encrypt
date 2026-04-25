use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use encrypt::{caesar_cipher, vigenere_cipher, columnar_cipher, xor_cipher, railfence_cipher};

pub fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("File not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file.");
    return contents;
}

fn prompt(label: &str) -> String {
    print!("{}", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let file_path = prompt("File Path: ");
    let plaintext = read_file(file_path);
    println!("{plaintext}");
    
    todo!()
}
