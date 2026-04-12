use encrypt::{read_file, columnar_cipher};

#[test]
fn encrypt_decrypt_short() {
    let key = vec![2, 0, 1];
    let plaintext = read_file(String::from("files/short.txt"));
    let ciphertext = columnar_cipher(plaintext.clone(), key.clone(), false);
    let decrypted = columnar_cipher(ciphertext, key, true);
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt_decrypt_message() {
    let key = vec![3, 1, 4, 0, 2];
    let plaintext = read_file(String::from("files/message3.txt"));
    let ciphertext = columnar_cipher(plaintext.clone(), key.clone(), false);
    let decrypted = columnar_cipher(ciphertext, key, true);
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt_decrypt_exact_fit() {
    // "abcdef" with key length 3 divides evenly — no padding needed
    let key = vec![1, 2, 0];
    let plaintext = String::from("abcdef");
    let ciphertext = columnar_cipher(plaintext.clone(), key.clone(), false);
    let decrypted = columnar_cipher(ciphertext, key, true);
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt_decrypt_single_col() {
    // Key of length 1 — cipher is a no-op
    let key = vec![0];
    let plaintext = String::from("hello");
    let ciphertext = columnar_cipher(plaintext.clone(), key.clone(), false);
    let decrypted = columnar_cipher(ciphertext, key, true);
    assert_eq!(decrypted, plaintext);
}
