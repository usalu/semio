//! 🧭️ Determines whether a PDF startxref points to a classic table or an indirect object.

use super::{is_delim, is_ws};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XrefSectionKind { Table, Stream }

pub struct XrefSectionProbe {
    position: usize,
    matched: usize,
    comment: bool,
    kind: Option<XrefSectionKind>,
}

impl XrefSectionProbe {
    pub fn new(offset: usize) -> Self { Self { position: offset, matched: 0, comment: false, kind: None } }

    /// 🔎️ Reads the first lexical word with at most one source byte inspection per unit.
    pub fn advance(&mut self, byte_at: impl Fn(usize) -> Option<u8>, grant: usize) -> (usize, Option<XrefSectionKind>) {
        const XREF: &[u8] = b"xref";
        let mut work = 0;
        while self.kind.is_none() && work < grant {
            let byte = byte_at(self.position);
            work += 1;
            match byte {
                _ if self.matched == XREF.len() => {
                    let ends_word = byte.is_none_or(|byte| is_ws(byte) || is_delim(byte));
                    self.kind = Some(if ends_word { XrefSectionKind::Table } else { XrefSectionKind::Stream });
                }
                Some(byte) if self.comment => self.comment = !matches!(byte, b'\n' | b'\r'),
                Some(b'%') if self.matched == 0 => self.comment = true,
                Some(byte) if self.matched == 0 && is_ws(byte) => {}
                Some(byte) if byte == XREF[self.matched] => self.matched += 1,
                _ => self.kind = Some(XrefSectionKind::Stream),
            }
            if byte.is_some() { self.position += 1; }
        }
        (work, self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_both_incremental_xref_fixtures_with_one_byte_grants() {
        for (source, expected) in [
            (include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf").as_slice(), XrefSectionKind::Table),
            (include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️incremental-free/💧️stream.pdf").as_slice(), XrefSectionKind::Stream),
        ] {
            let mut cursor = XrefSectionProbe::new(1002);
            loop {
                let (work, result) = cursor.advance(|position| source.get(position).copied(), 1);
                assert_eq!(work, 1);
                if let Some(actual) = result { assert_eq!(actual, expected); break }
            }
        }
    }
}
