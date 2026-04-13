use encrypt::caesar_cipher;

#[test]
fn test_caesar_encrypt_basic() {
    assert_eq!(
        caesar_cipher("Hello World!".to_string(), 3, false),
        "Khoor Zruog!"
    );
}

#[test]
fn test_caesar_decrypt_basic() {
    assert_eq!(
        caesar_cipher("Khoor Zruog!".to_string(), 3, true),
        "Hello World!"
    );
}

#[test]
fn test_caesar_encrypt_wrap() {
    assert_eq!(
        caesar_cipher("xyz".to_string(), 5, false),
        "cde"
    );
}

#[test]
fn test_caesar_decrypt_wrap() {
    assert_eq!(
        caesar_cipher("cde".to_string(), 5, true),
        "xyz"
    );
}

#[test]
fn test_caesar_negative_shift() {
    assert_eq!(
        caesar_cipher("abc".to_string(), -1, false),
        "zab"
    );
    assert_eq!(
        caesar_cipher("zab".to_string(), -1, true),
        "abc"
    );
}

#[test]
fn test_caesar_large_shift() {
    assert_eq!(
        caesar_cipher("abc".to_string(), 27, false),
        "bcd"
    );
    assert_eq!(
        caesar_cipher("bcd".to_string(), 27, true),
        "abc"
    );
}

#[test]
fn test_caesar_non_alphabetic() {
    assert_eq!(
        caesar_cipher("123 !@#".to_string(), 10, false),
        "123 !@#"
    );
    assert_eq!(
        caesar_cipher("123 !@#".to_string(), 10, true),
        "123 !@#"
    );
}
