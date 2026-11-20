//! 🧪 Compare COS byte strings for nested, escaped, and hexadecimal PDF syntax.

use super::{PdfByteStringCursor, StringStep};

fn octets(hex: &str) -> Vec<u8> {
    hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

#[test]
fn strings_resume_under_work_and_output_limits() {
    for row in include_str!("🧪️strings.txt").lines() {
        let (source, expected) = row.split_once(' ').unwrap();
        let source = octets(source);
        for budget in [1, 2, 7, 64] {
            for width in [1, 3, 8] {
                let mut cursor = PdfByteStringCursor::new(0);
                let mut bytes = Vec::new();
                for _ in 0..source.len() * 3 + 3 {
                    let mut scratch = [0; 8];
                    let receipt = cursor.advance(&source, &mut scratch[..width], budget);
                    assert!(receipt.work <= budget);
                    bytes.extend_from_slice(&scratch[..receipt.written]);
                    if receipt.status.unwrap() == StringStep::Complete { break; }
                }
                assert_eq!(cursor.position(), source.len());
                assert_eq!(bytes, octets(expected));
            }
        }
    }
}
