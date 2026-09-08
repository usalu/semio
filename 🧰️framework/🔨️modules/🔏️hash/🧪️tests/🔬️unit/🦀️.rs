
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
