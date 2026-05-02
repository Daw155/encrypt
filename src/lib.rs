pub fn caesar_cipher(str: String, key: String, decrypt: bool) -> Result<String, String> {
    // Error checking
    if key.is_empty() {
        return Err(String::from("Key cannot be empty"));
    }
    if key.parse::<i32>().is_err() {
        return Err(format!("'{}' is not a valid integer", key));
    }

    // Parse shift and normalize to [0, 25]
    let shift = key.parse::<i32>().unwrap();
    let mut actual_shift = shift % 26;
    if actual_shift < 0 {
        actual_shift += 26;
    }

    // Reverse the shift for decryption
    if decrypt {
        actual_shift = (26 - actual_shift) % 26;
    }

    // Result string
    let mut result = String::new();

    // Shift each alphabetic character, leaving non-alphabetic characters unchanged
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
    Ok(result)
}

pub fn vigenere_cipher(str: String, key: String, decrypt: bool) -> Result<String, String> {
    // Error checking
    if key.is_empty() {
        return Err(String::from("Key cannot be empty"));
    }
    if !key.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(String::from("Key must contain alphabetical characters only"));
    }

    // Result string
    let mut result = String::new();

    // Normalize key to lowercase and collect into a vec of chars
    let lowercase_key = key.to_lowercase();
    let key_chars : Vec<char> = lowercase_key.chars().collect();

    // Tracks position in the key; only advances on alphabetic characters
    let mut key_index = 0;

    // Shift each alphabetic character using the repeating key, leaving others unchanged
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

    Ok(result)
}

pub fn columnar_cipher(str: String, key: String, decrypt: bool) -> Result<String, String> {
    // Error checking
    if key.is_empty() {
        return Err(String::from("Key cannot be empty"));
    }
    if !key.chars().all(|c| c.is_numeric()) {
        return Err(String::from("Key must contain numerical characters only"));
    }
    let digits: Vec<usize> = key.chars().map(|c| c as usize - '0' as usize).collect();
    if digits.iter().any(|&d| d >= key.len()) {
        return Err(String::from("Each digit must be a valid column index (0 to key length - 1)"));
    }
    let mut seen = vec![false; key.len()];
    for &d in &digits {
        if seen[d] {
            return Err(String::from("Key must not contain duplicate digits"));
        }
        seen[d] = true;
    }

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

    return Ok(result);
}

pub fn xor_cipher(str: String, key: String, _decrypt: bool) -> Result<String, String> {
    // Error checking
    if key.is_empty() {
        return Err(String::from("Key cannot be empty"));
    }
    if !key.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(String::from("Key must contain alphabetical characters only"));
    }

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

    return Ok(result);
}

pub fn railfence_cipher(str: String, key: String, decrypt: bool) -> Result<String, String> {
    // Error checking
    if key.is_empty() {
        return Err(String::from("Key cannot be empty"));
    }
    if key.parse::<usize>().is_err() {
        return Err(format!("'{}' is not a valid integer", key));
    }

    // Parse fence height; return input unchanged if height <= 1 or text is empty
    let h = key.parse::<usize>().unwrap();
    if h <= 1 || str.is_empty() {
        return Ok(str);
    }

    let len = str.len();

    // Create a grid and mark which cells are on the zigzag rail pattern
    let mut grid = vec![vec!['\n'; len]; h];
    let mut row = 0;
    let mut moving_down = false;
    for col in 0..len {
        grid[row][col] = '*';
        if row == 0 || row == h - 1 {
            moving_down = !moving_down;
        }
        if moving_down {
            row += 1;
        } else {
            row -= 1;
        }
    }

    // Result string
    let mut result = String::new();

    if !decrypt {
        // Encryption
        let chars: Vec<char> = str.chars().collect();
        for col in 0..len {
            for r in 0..h {
                if grid[r][col] == '*' {
                    grid[r][col] = chars[col];
                }
            }
        }
        for r in 0..h {
            for c in 0..len {
                if grid[r][c] != '\n' && grid[r][c] != '*' {
                    result.push(grid[r][c]);
                }
            }
        }
    } else {
        // Decryption
        let mut text_chars = str.chars();
        for r in 0..h {
            for c in 0..len {
                if grid[r][c] == '*' {
                    if let Some(ch) = text_chars.next() {
                        grid[r][c] = ch;
                    }
                }
            }
        }
        for c in 0..len {
            for r in 0..h {
                if grid[r][c] != '\n' && grid[r][c] != '*' {
                    result.push(grid[r][c]);
                }
            }
        }
    }
    return Ok(result);
}