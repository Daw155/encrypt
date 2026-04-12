use std::fs::File;
use std::io::Read;

pub fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("File not found.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file.");
    return contents;
}


pub fn caesar_cipher(str: String, shift: i32, decrypt: bool) -> String {
    todo!()
}

pub fn vigenere_cipher(str: String, key: String, decrypt: bool) -> String { 
    todo!()
}

pub fn columnar_cipher(str: String, key: Vec<usize>, decrypt: bool) -> String {
    // Result string
    let mut result = String::new();

    // Create a grid to store the text in
    let mut grid: Vec<Vec<char>> = Vec::new();
    let rows: usize = str.len().div_ceil(key.len());
    let cols: usize = key.len();
    for _ in 0..rows {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..cols {
            row.push('\0');
        }
        grid.push(row);
    }


    let mut index = 0;
    if decrypt {
        // Decryption
        let remainder = str.len() % cols;
        for &c in &key {
            let col_len = if remainder == 0 || c < remainder { rows } else { rows - 1 };
            for r in 0..col_len {
                grid[r][c] = str.chars().nth(index).expect("OOB");
                index += 1;
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '\0' {
                    result.push(grid[r][c]);
                }
            }
        }
    } else {
        // Encryption
        for r in 0..rows {
            for c in 0..cols {
                if index >= str.len() {
                    break;
                }
                grid[r][c] = str.chars().nth(index).expect("OOB");
                index += 1;
            }
        }

        for c in key {
            for r in 0..rows {
                if grid[r][c] != '\0' {
                    result.push(grid[r][c]);
                }
            }
        }
    }

    return result;
}

pub fn xor_cipher(str: String, decrypt: bool) -> String {
    todo!()
}

pub fn railfence_cipher(str: String, height: i32, decrypt: bool) -> String {
    todo!()
}
