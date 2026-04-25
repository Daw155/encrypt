pub fn caesar_cipher(str: String, key: String, decrypt: bool) -> String {
    let shift = key.parse::<i32>().unwrap();
    let mut result = String::new();
    let mut actual_shift = shift % 26;
    if actual_shift < 0 {
        actual_shift += 26;
    }
    if decrypt {
        actual_shift = (26 - actual_shift) % 26;
    }
    for c in str.chars() {
        if c.is_ascii_lowercase() {
            let offset = c as u8 - ('a' as u8);
            let new_offset = (offset + actual_shift as u8) % 26;
            result.push((('a' as u8) + new_offset) as char);
        } else if c.is_ascii_uppercase() {
            let offset = c as u8 - ('A' as u8);
            let new_offset = (offset + actual_shift as u8) % 26;
            result.push((('A' as u8) + new_offset) as char);
        } else {
            result.push(c);
        }
    }
    
    return result;
}

pub fn vigenere_cipher(str: String, key: String, decrypt: bool) -> String { 
    let mut result = String::new();
    let lowercase_key = key.to_lowercase();
    let key_chars : Vec<char> = lowercase_key.chars().collect();
    let mut key_index = 0;
    for c in str.chars() {
        if c.is_ascii_alphabetic() {
            let shift = (key_chars[key_index % key_chars.len()] as i32) - ('a' as i32);
            let mut actual_shift = shift % 26;
            if decrypt {
                actual_shift = (26 - actual_shift) % 26;
            }
            if c.is_ascii_lowercase() {
                let offset = c as u8 - ('a' as u8);
                let new_offset = (offset + actual_shift as u8) % 26;
                result.push((('a' as u8) + new_offset) as char)
            } else {
                let offset = c as u8 - ('A' as u8);
                let new_offset = (offset + actual_shift as u8) % 26;
                result.push((('A' as u8) + new_offset) as char)
            }
            key_index += 1;
        } else {
            result.push(c);
        }
    }

    return result;
}

pub fn columnar_cipher(str: String, key: String, decrypt: bool) -> String {
    // Result string
    let mut result = String::new();

    // Convert str into a vec of chars
    let str_chars: Vec<char> = str.chars().collect();

    // Convert key into a vec of nums
    let mut vec = Vec::new();
    for c in key.chars() {
        vec.push(c as usize - '0' as usize);
    }

    // Create a grid to store the text in
    let mut grid: Vec<Vec<char>> = Vec::new();
    let rows: usize = str.len().div_ceil(vec.len());
    let cols: usize = vec.len();
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
        for &c in &vec {
            let col_len = if remainder == 0 || c < remainder { rows } else { rows - 1 };
            for r in 0..col_len {
                grid[r][c] = str_chars[index];
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
                grid[r][c] = str_chars[index];
                index += 1;
            }
        }

        for c in vec {
            for r in 0..rows {
                if grid[r][c] != '\0' {
                    result.push(grid[r][c]);
                }
            }
        }
    }

    return result;
}

pub fn xor_cipher(str: String, key: String, _decrypt: bool) -> String {
    // Result String
    let mut result: String = String::new();

    // Gathers all of the chars of key into a vec
    let key_chars: Vec<char> = key.chars().collect();

    // Performs the xor operation on each char in str
    for (i, c) in str.chars().enumerate() {
        // Grabs the key character associated with c; performs a modular loop if key.len() < str.len()
        let key_c = key_chars[i % key_chars.len()];
        // Bitwise XOR operation (from: https://doc.rust-lang.org/std/ops/trait.BitXor.html)
        result.push(char::from_u32((c as u32) ^ (key_c as u32)).expect("Conversion Error"));
    }

    return result;
}

pub fn railfence_cipher(str: String, height: i32, decrypt: bool) -> String {
    todo!()
}