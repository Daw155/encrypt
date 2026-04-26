use encrypt::railfence_cipher;

#[test]
fn test_railfence_encrypt() {
    let t1 = String::from("WEAREDISCOVEREDRUNATONCE");
    let encrypted = railfence_cipher(t1, 3, false);
    assert_eq!(encrypted, "WECRUOERDSOEERNTNEAIVDAC");

    let t2 = String::from("HELLOWORLD");
    let encrypted2 = railfence_cipher(t2, 3, false);
    assert_eq!(encrypted2, "HOLELWRDLO");

    let t3 = String::from("WEAREDISCOVEREDRUNATONCE");
    let encrypted3 = railfence_cipher(t3, 6, false);
    // W.........V.........O
    // .E.......O.E.......T.N
    // ..A.....C...R.....A...C
    // ...R...S.....E...N.....E
    // ....E.I.......D.U.......
    // .....D.........R........
    assert_eq!(encrypted3, "WVOEOETNACRACRSENEEIDUDR");
}

#[test]
fn test_railfence_decrypt() {
    let ciphertext = String::from("WECRUOERDSOEERNTNEAIVDAC");
    let decrypted = railfence_cipher(ciphertext.clone(), 3, true);
    assert_eq!(decrypted, "WEAREDISCOVEREDRUNATONCE");

    let ciphertext2 = String::from("HOLELWRDLO");
    let decrypted2 = railfence_cipher(ciphertext2.clone(), 3, true);
    assert_eq!(decrypted2, "HELLOWORLD");

    let ciphertext3 = String::from("WVOEOETNACRACRSENEEIDUDR");
    let decrypted3 = railfence_cipher(ciphertext3.clone(), 6, true);
    assert_eq!(decrypted3, "WEAREDISCOVEREDRUNATONCE");
}
