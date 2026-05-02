use encrypt::xor_cipher;

#[test]
fn test_xor_encrypt_basic() {
    // Non-alphabetical key should return an error
    assert!(xor_cipher("ABC".to_string(), " ".to_string(), false).is_err());
}

#[test]
fn test_xor_decrypt_basic() {
    // Non-alphabetical key should return an error
    assert!(xor_cipher("abc".to_string(), " ".to_string(), true).is_err());
}

#[test]
fn test_xor_symmetric() {
    // XOR is its own inverse: encrypting twice returns the original
    let plaintext = "Hello, World!".to_string();
    let key = "secret".to_string();
    let encrypted = xor_cipher(plaintext.clone(), key.clone(), false).unwrap();
    assert_eq!(xor_cipher(encrypted, key, true).unwrap(), plaintext);
}

#[test]
fn test_xor_decrypt_flag_ignored() {
    // Encrypting then decrypting returns the original
    let plaintext = "Hello".to_string();
    let key = "key".to_string();
    let encrypted = xor_cipher(plaintext.clone(), key.clone(), false).unwrap();
    assert_eq!(xor_cipher(encrypted, key, true).unwrap(), plaintext);
}

#[test]
fn test_xor_key_cycles() {
    // XOR with itself produces all null bytes
    let plaintext = "ABCABC".to_string();
    let key = "ABC".to_string();
    // Encrypted output is base64 of null bytes
    assert_eq!(
        xor_cipher(plaintext, key, false).unwrap(),
        "AAAAAAAA"
    );
}

#[test]
fn test_xor_key_cycles_roundtrip() {
    // Key shorter than text should cycle and still roundtrip correctly
    let plaintext = "Hello World".to_string();
    let key = "ab".to_string();
    let encrypted = xor_cipher(plaintext.clone(), key.clone(), false).unwrap();
    assert_eq!(xor_cipher(encrypted, key, true).unwrap(), plaintext);
}

#[test]
fn test_xor_empty_string() {
    assert_eq!(
        xor_cipher("".to_string(), "key".to_string(), false).unwrap(),
        ""
    );
}

#[test]
fn test_xor_single_char() {
    let plaintext = "A".to_string();
    let key = "A".to_string();
    // Encrypted output is base64 of a null byte
    assert_eq!(xor_cipher(plaintext, key, false).unwrap(), "AA==");
}

#[test]
fn test_xor_empty_key() {
    assert!(xor_cipher("hello".to_string(), "".to_string(), false).is_err());
}

#[test]
fn test_xor_non_alpha_key() {
    assert!(xor_cipher("hello".to_string(), "k3y".to_string(), false).is_err());
}
