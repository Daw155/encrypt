use encrypt::caesar_cipher;

#[test]
fn test_caesar_encrypt_basic() {
    assert_eq!(
        caesar_cipher("Hello World!".to_string(), "3".to_string(), false),
        "Khoor Zruog!"
    );
}

#[test]
fn test_caesar_decrypt_basic() {
    assert_eq!(
        caesar_cipher("Khoor Zruog!".to_string(), "3".to_string(), true),
        "Hello World!"
    );
}

#[test]
fn test_caesar_encrypt_wrap() {
    assert_eq!(
        caesar_cipher("xyz".to_string(), "5".to_string(), false),
        "cde"
    );
}

#[test]
fn test_caesar_decrypt_wrap() {
    assert_eq!(
        caesar_cipher("cde".to_string(), "5".to_string(), true),
        "xyz"
    );
}

#[test]
fn test_caesar_negative_shift() {
    assert_eq!(
        caesar_cipher("abc".to_string(), "-1".to_string(), false),
        "zab"
    );
    assert_eq!(
        caesar_cipher("zab".to_string(), "-1".to_string(), true),
        "abc"
    );
}

#[test]
fn test_caesar_large_shift() {
    assert_eq!(
        caesar_cipher("abc".to_string(), "27".to_string(), false),
        "bcd"
    );
    assert_eq!(
        caesar_cipher("bcd".to_string(), "27".to_string(), true),
        "abc"
    );
}

#[test]
fn test_caesar_non_alphabetic() {
    assert_eq!(
        caesar_cipher("123 !@#".to_string(), "10".to_string(), false),
        "123 !@#"
    );
    assert_eq!(
        caesar_cipher("123 !@#".to_string(), "10".to_string(), true),
        "123 !@#"
    );
}
