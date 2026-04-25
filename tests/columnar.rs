use encrypt::columnar_cipher;

#[test]
fn encrypt_decrypt_short() {
    let plaintext = "hello world".to_string();
    let ciphertext = columnar_cipher(plaintext.clone(), "201".to_string(), false);
    let decrypted = columnar_cipher(ciphertext, "201".to_string(), true);
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt_decrypt_message() {
    let plaintext = "We are discovered. Run at once!".to_string();
    let ciphertext = columnar_cipher(plaintext.clone(), "31402".to_string(), false);
    let decrypted = columnar_cipher(ciphertext, "31402".to_string(), true);
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt_decrypt_exact_fit() {
    // "abcdef" with key length 3 divides evenly — no padding needed
    let plaintext = String::from("abcdef");
    let ciphertext = columnar_cipher(plaintext.clone(), "120".to_string(), false);
    let decrypted = columnar_cipher(ciphertext, "120".to_string(), true);
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt_decrypt_single_col() {
    // Key of length 1 — cipher is a no-op
    let plaintext = String::from("hello");
    let ciphertext = columnar_cipher(plaintext.clone(), "0".to_string(), false);
    let decrypted = columnar_cipher(ciphertext, "0".to_string(), true);
    assert_eq!(decrypted, plaintext);
}
