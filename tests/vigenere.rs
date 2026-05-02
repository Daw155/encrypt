use encrypt::vigenere_cipher;

#[test]
fn test_vigenere_encrypt_basic() {
    assert_eq!(
        vigenere_cipher("Hello World!".to_string(), "dog".to_string(), false).unwrap(),
        "Ksroc Crfrg!"
    );
}
#[test]
fn test_vigenere_decrypt_basic() {
    assert_eq!(
        vigenere_cipher("Ksroc Crfrg!".to_string(), "dog".to_string(), true).unwrap(),
        "Hello World!"
    );
}
#[test]
fn test_vigenere_encrypt_wrap() {
    assert_eq!(
        vigenere_cipher("xyz".to_string(), "dog".to_string(), false).unwrap(),
        "amf"
    );
}
#[test]
fn test_vigenere_decrypt_wrap() {
    assert_eq!(
        vigenere_cipher("amf".to_string(), "dog".to_string(), true).unwrap(),
        "xyz"
    );
}
#[test]
fn test_vigenere_key_cycles() {
    assert_eq!(
        vigenere_cipher("abcdef".to_string(), "ab".to_string(), false).unwrap(),
        "acceeg"
    );
    assert_eq!(
        vigenere_cipher("acceeg".to_string(), "ab".to_string(), true).unwrap(),
        "abcdef"
    );
}
#[test]
fn test_vigenere_non_alphabetic() {
    assert_eq!(
        vigenere_cipher("123 !@#".to_string(), "dog".to_string(), false).unwrap(),
        "123 !@#"
    );
    assert_eq!(
        vigenere_cipher("123 !@#".to_string(), "dog".to_string(), true).unwrap(),
        "123 !@#"
    );
}
#[test]
fn test_vigenere_key_case_insensitive() {
    assert_eq!(
        vigenere_cipher("Hello".to_string(), "DOG".to_string(), false).unwrap(),
        vigenere_cipher("Hello".to_string(), "dog".to_string(), false).unwrap(),
    );
}
#[test]
fn test_vigenere_empty_key() {
    assert!(vigenere_cipher("abc".to_string(), "".to_string(), false).is_err());
}
#[test]
fn test_vigenere_non_alpha_key() {
    assert!(vigenere_cipher("abc".to_string(), "d0g".to_string(), false).is_err());
}
