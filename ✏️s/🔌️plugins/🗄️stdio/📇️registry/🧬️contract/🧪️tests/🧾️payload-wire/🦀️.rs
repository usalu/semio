use super::*;
use base64::Engine;

#[test]
fn standard_base64_matches_the_reference_implementation() {
    for bytes in [b"".as_slice(), b"f", b"fo", b"foo", b"foobar", &[0, 127, 128, 255]] {
        assert_eq!(base64_standard(bytes), base64::engine::general_purpose::STANDARD.encode(bytes));
    }
}
