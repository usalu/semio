use super::*;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn md5_known_answers() {
    assert_eq!(hex(&md5(b"")), "d41d8cd98f00b204e9800998ecf8427e");
    assert_eq!(hex(&md5(b"The quick brown fox jumps over the lazy dog")), "9e107d9d372bb6826bd81d3542a419d6");
    assert_eq!(hex(&md5(&[b'a'; 1000])), "cabe45dcc9ae5b66ba86600cca6b8ba8");
}

#[test]
fn sha2_known_answers() {
    assert_eq!(hex(&sha256(b"abc")), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    assert_eq!(hex(&sha384(b"abc")), "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7");
    assert_eq!(hex(&sha512(b"abc")), "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f");
    assert_eq!(hex(&sha512(b"")), "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
}

#[test]
fn rc4_known_answer_and_symmetry() {
    assert_eq!(hex(&rc4(b"Key", b"Plaintext")), "bbf316e8d940af0ad3");
    assert_eq!(rc4(b"Key", &rc4(b"Key", b"round trip")), b"round trip");
}

#[test]
fn aes_fips197_vectors_and_cbc_round_trip() {
    let key128: Vec<u8> = (0u8..16).collect();
    let plain: [u8; 16] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    let aes = AesKey::new(&key128);
    let cipher = aes.encrypt_block(&plain);
    assert_eq!(hex(&cipher), "69c4e0d86a7b0430d8cdb78070b4c55a");
    assert_eq!(aes.decrypt_block(&cipher), plain);
    let key256: Vec<u8> = (0u8..32).collect();
    let aes = AesKey::new(&key256);
    let cipher = aes.encrypt_block(&plain);
    assert_eq!(hex(&cipher), "8ea2b7ca516745bfeafc49904b496089");
    assert_eq!(aes.decrypt_block(&cipher), plain);
    let data = b"seventeen bytes!!".to_vec();
    let sealed = aes_cbc_encrypt(&key128, [9u8; 16], &data);
    assert_eq!(sealed.len(), 16 + 32);
    assert_eq!(aes_cbc_decrypt(&key128, &sealed), data);
}

#[test]
fn every_algorithm_seals_and_opens_with_user_and_owner_passwords() {
    let document_id = b"0123456789abcdef";
    for algorithm in [PdfEncryptionAlgorithm::Rc4_40, PdfEncryptionAlgorithm::Rc4_128, PdfEncryptionAlgorithm::Aes128, PdfEncryptionAlgorithm::Aes256] {
        let parameters = PdfEncryption { algorithm, permissions: -3904, user_password: "user".into(), owner_password: Some("owner".into()), encrypt_metadata: true };
        let (sealer, dictionary) = seal_standard_security(&parameters, document_id, b"seed");
        let object = PdfObject::Dict(vec![PdfDictEntry::new("S", PdfObject::Str(b"secret string".to_vec())), PdfDictEntry::new("T", PdfObject::Stream { dict: Vec::new(), data: b"stream body".to_vec(), filters: Vec::new() })]);
        let sealed = sealer.encrypt_object(object.clone(), 7, 0);
        assert_ne!(sealed, object);
        for password in ["user", "owner"] {
            let (opener, parsed) = open_standard_security(&dictionary, document_id, password).unwrap_or_else(|error| panic!("{algorithm:?} with {password}: {error}"));
            assert_eq!(parsed.algorithm, algorithm);
            assert_eq!(opener.decrypt_object(sealed.clone(), 7, 0), object, "{algorithm:?} {password}");
        }
        assert!(open_standard_security(&dictionary, document_id, "wrong").is_err());
    }
}
