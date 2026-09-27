
use super::*;

#[test]
fn hashes_bytes_deterministically() {
    let first = hash_bytes(b"hello");
    let second = hash_bytes(b"hello");
    assert_eq!(first, second);
    assert_ne!(first, hash_bytes(b"world"));
}

#[test]
fn sha256_matches_nist_vectors_and_segmented_input() {
    assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    let mut segmented = Sha256::new();
    segmented.update(b"a");
    segmented.update(b"b");
    segmented.update(b"c");
    assert_eq!(hex_lower(&segmented.finalize()), sha256_hex(b"abc"));
    let long = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    assert_eq!(sha256_hex(long), "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
    let mut segmented_long = Sha256::new();
    for chunk in long.chunks(7) {
        segmented_long.update(chunk);
    }
    assert_eq!(hex_lower(&segmented_long.finalize()), sha256_hex(long));
}

#[test]
fn normalizes_hash_numbers() {
    assert_eq!(format_number_for_hash(-0.0), "0");
    assert_eq!(format_number_for_hash(42.0), "42");
    assert_eq!(format_number_for_hash(1.25), "1.25");
}

#[test]
fn separates_hash_parts_with_a_delimiter() {
    assert_ne!(hash_parts(&["ab", "c"]), hash_parts(&["a", "bc"]));
}

#[test]
fn orders_merkle_children_deterministically() {
    assert_eq!(merkle_node(&["root"], vec!["child-b".into(), "child-a".into()]), merkle_node(&["root"], vec!["child-a".into(), "child-b".into()]),);
}

#[test]
fn normalizes_special_hash_numbers() {
    assert_eq!(format_number_for_hash(f64::NAN), "nan");
    assert_eq!(format_number_for_hash(f64::INFINITY), "inf");
    assert_eq!(format_number_for_hash(f64::NEG_INFINITY), "-inf");
}

//#region 🧪️Blake3Oracle
/// 🧪️ `blake3` lives ONLY in `[dev-dependencies]` here — the one place in the framework
/// allowed to keep it, purely as the differential oracle proving our BLAKE3 is byte-exact.
fn blake3_test_input(len: usize) -> Vec<u8> {
    (0..len).map(|index| (index % 251) as u8).collect()
}

#[test]
fn hash_bytes_agrees_with_the_blake3_oracle_across_lengths() {
    for len in [0, 1, 2, 3, 63, 64, 65, 1023, 1024, 1025, 2048, 2049, 3072, 3073, 4096, 4097, 5120, 102400] {
        let input = blake3_test_input(len);
        assert_eq!(hash_bytes(&input), blake3::hash(&input).to_hex().to_string(), "mismatch at len={len}");
    }
}

#[test]
fn hash_bytes_agrees_with_the_blake3_oracle_on_ad_hoc_samples() {
    for sample in [b"".as_slice(), b"abc", b"consumer of the puzzle plugin", &[0u8; 128], &[0xffu8; 1]] {
        assert_eq!(hash_bytes(sample), blake3::hash(sample).to_hex().to_string());
    }
}

#[test]
fn hasher_matches_one_shot_hash_for_segmented_updates() {
    let mut segmented = Hasher::new();
    for chunk in blake3_test_input(5120).chunks(37) {
        segmented.update(chunk);
    }
    assert_eq!(segmented.finalize().to_hex(), hash_bytes(&blake3_test_input(5120)));
}

#[test]
fn hasher_agrees_with_the_blake3_oracle_for_segmented_updates() {
    let input = blake3_test_input(102400);
    let mut ours = Hasher::new();
    let mut oracle = blake3::Hasher::new();
    for chunk in input.chunks(777) {
        ours.update(chunk);
        oracle.update(chunk);
    }
    assert_eq!(ours.finalize().as_bytes(), oracle.finalize().as_bytes());
}
//#endregion 🧪️Blake3Oracle

/// 🏎️ Every SHA-256 compression this machine can select agrees with the portable FIPS 180-4 rounds on
/// NIST's blocks and on pseudo-random states and blocks, so hardware selection never changes a digest.
#[test]
fn sha256_hardware_compression_agrees_with_the_portable_rounds() {
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for _ in 0..20_000 {
        let mut state = [0u32; 8];
        state.iter_mut().for_each(|word| *word = next() as u32);
        let mut block = [0u8; 64];
        block.iter_mut().for_each(|byte| *byte = next() as u8);
        let (mut selected, mut portable) = (state, state);
        sha256_compress(&mut selected, &block);
        sha256_compress_portable(&mut portable, &block);
        assert_eq!(selected, portable, "state {state:08x?} block {block:02x?}");
    }
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

/// 🔬️ Third-party oracle: the `sha2` crate agrees with this crate's `Sha256` on every length class
/// around the block and padding boundaries, whole and in uneven segments.
#[test]
fn sha256_agrees_with_the_sha2_oracle_across_lengths() {
    use sha2::Digest as _;
    for length in (0..=260).chain([511, 512, 513, 4_095, 4_096, 65_537, 1_000_003]) {
        let input: Vec<u8> = (0..length).map(|index| (index as u32).wrapping_mul(2_654_435_761).rotate_left(7) as u8).collect();
        let oracle: [u8; 32] = sha2::Sha256::digest(&input).into();
        assert_eq!(Sha256::digest(&input), oracle, "length {length}");
        let mut segmented = Sha256::new();
        for chunk in input.chunks(37) {
            segmented.update(chunk);
        }
        assert_eq!(segmented.finalize(), oracle, "segmented length {length}");
    }
}

/// 🔑️ RFC 2104 HMAC and RFC 8018 PBKDF2 composed over the third-party `sha2` oracle.
fn oracle_hmac(key: &[u8], message: &[u8]) -> [u8; 32] {
    use sha2::Digest;
    let mut block = [0u8; 64];
    if key.len() > 64 {
        block[..32].copy_from_slice(&sha2::Sha256::digest(key));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let inner = sha2::Sha256::new().chain_update(block.map(|byte| byte ^ 0x36)).chain_update(message).finalize();
    sha2::Sha256::new().chain_update(block.map(|byte| byte ^ 0x5c)).chain_update(inner).finalize().into()
}

#[test]
fn hmac_and_pbkdf2_agree_with_the_rfc_composition_over_the_sha2_oracle() {
    for (key_len, message_len) in [(0, 0), (3, 43), (32, 32), (64, 64), (65, 7), (200, 1000)] {
        let key: Vec<u8> = (0..key_len).map(|index| (index * 7 + 1) as u8).collect();
        let message: Vec<u8> = (0..message_len).map(|index| (index * 13 + 5) as u8).collect();
        let mac = HmacSha256::new(&key);
        assert_eq!(mac.mac(&message), oracle_hmac(&key, &message), "hmac key {key_len} message {message_len}");
        if let Ok(fixed) = <[u8; 32]>::try_from(message.as_slice()) {
            assert_eq!(mac.mac32(&fixed), mac.mac(&message), "mac32 key {key_len}");
        }
    }
    let (password, salt) = (b"correct horse battery staple".as_slice(), [0x2au8; 16]);
    let mut seed = salt.to_vec();
    seed.extend_from_slice(&1u32.to_be_bytes());
    let mut block = oracle_hmac(password, &seed);
    let mut expected = block;
    for _ in 1..1_000 {
        block = oracle_hmac(password, &block);
        for (sum, byte) in expected.iter_mut().zip(block) {
            *sum ^= byte;
        }
    }
    assert_eq!(pbkdf2_sha256(password, &salt, 1_000), expected);
}
