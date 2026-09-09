
use super::*;

/// 📐️ RFC 4648 §10 vectors, kept as a language-agnostic fixture so any implementation in any
/// language can be checked against the same table.
#[test]
fn matches_rfc4648_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️rfc4648-base64-vectors.json")).expect("fixture JSON");
    for case in fixture["cases"].as_array().expect("cases array") {
        let input = case["input_utf8"].as_str().expect("input_utf8");
        let expected = case["encoded"].as_str().expect("encoded");
        assert_eq!(base64_standard_encode(input), expected, "encode({input:?})");
        assert_eq!(base64_standard_decode(expected).expect("decode fixture"), input.as_bytes(), "decode({expected:?})");
    }
}

#[test]
fn round_trips_every_byte_and_chunk_remainder() {
    let raw: Vec<u8> = (0u8..=u8::MAX).chain([0, 1, 2, 3, 4]).collect();
    let encoded = base64_standard_encode(&raw);
    assert_eq!(base64_standard_decode(encoded.as_bytes()), Ok(raw));
}

#[test]
fn rejects_malformed_and_noncanonical_inputs() {
    assert_eq!(base64_standard_decode(b"Zg" as &[u8]), Err(Base64Error::InvalidLength));
    assert_eq!(base64_standard_decode(b"Z g=" as &[u8]), Err(Base64Error::InvalidByte { index: 1, byte: b' ' }));
    assert_eq!(base64_standard_decode(b"=m9v" as &[u8]), Err(Base64Error::InvalidPadding));
    assert_eq!(base64_standard_decode(b"Zm=v" as &[u8]), Err(Base64Error::InvalidPadding));
    assert_eq!(base64_standard_decode(b"Zg==Zm8=" as &[u8]), Err(Base64Error::InvalidPadding));
    assert_eq!(base64_standard_decode(b"Zh==" as &[u8]), Err(Base64Error::NonCanonicalTrailingBits));
    assert_eq!(base64_standard_decode(b"Zm9=" as &[u8]), Err(Base64Error::NonCanonicalTrailingBits));
}

/// 🎲️ A tiny deterministic linear-congruential generator — no `rand` dependency, but still
/// exercises many distinct, reproducible byte strings across a differential run.
struct Lcg(u64);

impl Lcg {
    fn next_byte(&mut self) -> u8 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 33) as u8
    }
}

/// 🔬️ Differential oracle: round-trips deterministic pseudo-random byte strings of every
/// length from 0 to 128 through both this codec and the third-party `base64` crate (a
/// dev-only dependency, never a runtime one) and asserts byte-for-byte agreement both ways.
#[test]
fn matches_third_party_base64_oracle() {
    use base64::Engine as _;
    let oracle = base64::engine::general_purpose::STANDARD;
    let mut lcg = Lcg(0x9E3779B97F4A7C15);
    for length in 0..=128usize {
        let bytes: Vec<u8> = (0..length).map(|_| lcg.next_byte()).collect();
        let ours_encoded = base64_standard_encode(&bytes);
        let oracle_encoded = oracle.encode(&bytes);
        assert_eq!(ours_encoded, oracle_encoded, "encode mismatch at length {length}");
        let oracle_decoded = oracle.decode(&ours_encoded).expect("oracle decode of our encoding");
        assert_eq!(oracle_decoded, bytes, "oracle decode of our encoding at length {length}");
        let ours_decoded = base64_standard_decode(&oracle_encoded).expect("our decode of oracle encoding");
        assert_eq!(ours_decoded, bytes, "our decode of oracle encoding at length {length}");
    }
}
