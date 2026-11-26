use super::{character, JsonReadSource};
use std::cell::Cell;

struct Counted<'a> {
    bytes: &'a [u8],
    reads: Cell<usize>,
}

impl JsonReadSource for Counted<'_> {
    fn byte_len(&self) -> usize { self.bytes.len() }
    fn byte_at(&self, index: usize) -> Option<u8> {
        self.reads.set(self.reads.get() + 1);
        self.bytes.get(index).copied()
    }
}

#[test]
fn unicode_scalars_read_each_original_byte_once() {
    let inputs: Vec<String> = serde_json::from_str(include_str!("🧫️utf8.json")).unwrap();
    for input in &inputs {
        let source = Counted { bytes: input.as_bytes(), reads: Cell::new(0) };
        for (offset, expected) in input.char_indices() {
            assert_eq!(character(&source, offset).unwrap(), expected);
        }
        assert_eq!(source.reads.get(), input.len());
    }
    for invalid in [b"\x80".as_slice(), b"\xc0\xaf", b"\xe2\x82", b"\xed\xa0\x80", b"\xf4\x90\x80\x80"] {
        assert!(character(invalid, 0).is_err());
    }
}
