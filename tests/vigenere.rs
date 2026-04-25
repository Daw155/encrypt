use encrypt::vigenere_cipher;

#[test]
fn test_vigenere_encrypt_basic() {
    assert_eq!(
        vigenere_cipher("Hello World!".to_string(), "dog".to_string(), false),
        "Ksroc Crfrg!"
    );
}
#[test]
fn test_vigenere_decrypt_basic() {
    assert_eq!(
        vigenere_cipher("Ksroc Crfrg!".to_string(), "dog".to_string(), true),
        "Hello World!"
    );
}
#[test]
fn test_vigenere_encrypt_wrap() {
    assert_eq!(
        vigenere_cipher("xyz".to_string(), "dog".to_string(), false),
        "amf"
    );
}
#[test]
fn test_vigenere_decrypt_wrap() {
    assert_eq!(
        vigenere_cipher("amf".to_string(), "dog".to_string(), true),
        "xyz"
    );
}
#[test]
fn test_vigenere_key_cycles() {
    assert_eq!(
        vigenere_cipher("abcdef".to_string(), "ab".to_string(), false),
        "acceeg"
    );
    assert_eq!(
        vigenere_cipher("acceeg".to_string(), "ab".to_string(), true),
        "abcdef"
    );
}
#[test]
fn test_vigenere_non_alphabetic() {
    assert_eq!(
        vigenere_cipher("123 !@#".to_string(), "dog".to_string(), false),
        "123 !@#"
    );
    assert_eq!(
        vigenere_cipher("123 !@#".to_string(), "dog".to_string(), true),
        "123 !@#"
    );
}
#[test]
fn test_vigenere_key_case_insensitive() {
    assert_eq!(
        vigenere_cipher("Hello".to_string(), "DOG".to_string(), false),
        vigenere_cipher("Hello".to_string(), "dog".to_string(), false),
    );
}
