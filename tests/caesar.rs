use encrypt::caesar_cipher;

#[test]
fn test_caesar_encrypt_basic() {
    assert_eq!(
        caesar_cipher("Hello World!".to_string(), "3".to_string(), false).unwrap(),
        "Khoor Zruog!"
    );
}

#[test]
fn test_caesar_decrypt_basic() {
    assert_eq!(
        caesar_cipher("Khoor Zruog!".to_string(), "3".to_string(), true).unwrap(),
        "Hello World!"
    );
}

#[test]
fn test_caesar_encrypt_wrap() {
    assert_eq!(
        caesar_cipher("xyz".to_string(), "5".to_string(), false).unwrap(),
        "cde"
    );
}

#[test]
fn test_caesar_decrypt_wrap() {
    assert_eq!(
        caesar_cipher("cde".to_string(), "5".to_string(), true).unwrap(),
        "xyz"
    );
}

#[test]
fn test_caesar_negative_shift() {
    assert_eq!(
        caesar_cipher("abc".to_string(), "-1".to_string(), false).unwrap(),
        "zab"
    );
    assert_eq!(
        caesar_cipher("zab".to_string(), "-1".to_string(), true).unwrap(),
        "abc"
    );
}

#[test]
fn test_caesar_large_shift() {
    assert_eq!(
        caesar_cipher("abc".to_string(), "27".to_string(), false).unwrap(),
        "bcd"
    );
    assert_eq!(
        caesar_cipher("bcd".to_string(), "27".to_string(), true).unwrap(),
        "abc"
    );
}

#[test]
fn test_caesar_non_alphabetic() {
    assert_eq!(
        caesar_cipher("123 !@#".to_string(), "10".to_string(), false).unwrap(),
        "123 !@#"
    );
    assert_eq!(
        caesar_cipher("123 !@#".to_string(), "10".to_string(), true).unwrap(),
        "123 !@#"
    );
}

#[test]
fn test_caesar_empty_key() {
    assert!(caesar_cipher("abc".to_string(), "".to_string(), false).is_err());
}

#[test]
fn test_caesar_non_integer_key() {
    assert!(caesar_cipher("abc".to_string(), "abc".to_string(), false).is_err());
}
