use encrypt::xor_cipher;

#[test]
fn test_xor_encrypt_basic() {
    // 'A' ^ ' ' = 'a', 'B' ^ ' ' = 'b', 'C' ^ ' ' = 'c'
    assert_eq!(
        xor_cipher("ABC".to_string(), " ".to_string(), false),
        "abc"
    );
}

#[test]
fn test_xor_decrypt_basic() {
    assert_eq!(
        xor_cipher("abc".to_string(), " ".to_string(), true),
        "ABC"
    );
}

#[test]
fn test_xor_symmetric() {
    // XOR is its own inverse: encrypting twice returns the original
    let plaintext = "Hello, World!".to_string();
    let key = "secret".to_string();
    let encrypted = xor_cipher(plaintext.clone(), key.clone(), false);
    assert_eq!(xor_cipher(encrypted, key, true), plaintext);
}

#[test]
fn test_xor_decrypt_flag_ignored() {
    // The decrypt flag has no effect since XOR is symmetric
    let text = "Hello".to_string();
    let key = "key".to_string();
    assert_eq!(
        xor_cipher(text.clone(), key.clone(), false),
        xor_cipher(text, key, true)
    );
}

#[test]
fn test_xor_key_cycles() {
    // XOR with itself produces all null bytes
    let plaintext = "ABCABC".to_string();
    let key = "ABC".to_string();
    assert_eq!(
        xor_cipher(plaintext, key, false),
        "\0\0\0\0\0\0"
    );
}

#[test]
fn test_xor_key_cycles_roundtrip() {
    // Key shorter than text should cycle and still roundtrip correctly
    let plaintext = "Hello World".to_string();
    let key = "ab".to_string();
    let encrypted = xor_cipher(plaintext.clone(), key.clone(), false);
    assert_eq!(xor_cipher(encrypted, key, false), plaintext);
}

#[test]
fn test_xor_empty_string() {
    assert_eq!(
        xor_cipher("".to_string(), "key".to_string(), false),
        ""
    );
}

#[test]
fn test_xor_single_char() {
    let plaintext = "A".to_string();
    let key = "A".to_string();
    assert_eq!(xor_cipher(plaintext, key, false), "\0");
}
