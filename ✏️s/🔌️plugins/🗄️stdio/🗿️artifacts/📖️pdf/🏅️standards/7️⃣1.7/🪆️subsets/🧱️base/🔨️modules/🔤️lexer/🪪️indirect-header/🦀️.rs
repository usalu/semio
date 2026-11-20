//! 🪪️ Reads the two identifiers and keyword of an indirect PDF object.

use super::{is_delim, is_ws};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::ObjRef;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderStep {
    More,
    Complete { id: ObjRef, body_offset: usize },
}

#[derive(Clone, Copy)]
enum State {
    StartNumber,
    Number,
    StartGeneration,
    Generation,
    Keyword(usize),
    Complete,
}

pub struct IndirectHeaderCursor {
    position: usize,
    state: State,
    number: u32,
    generation: u16,
    comment: bool,
}

impl IndirectHeaderCursor {
    pub fn new(position: usize) -> Self {
        Self { position, state: State::StartNumber, number: 0, generation: 0, comment: false }
    }

    pub fn position(&self) -> usize { self.position }

    /// 🧾️ Inspects at most one source byte per unit of work, retaining split tokens across grants.
    pub fn advance(&mut self, source: &[u8], grant: usize) -> Result<(usize, HeaderStep), &'static str> {
        let mut inspected = 0;
        loop {
            if matches!(self.state, State::Complete) {
                let id = ObjRef { num: self.number, gen: self.generation };
                return Ok((inspected, HeaderStep::Complete { id, body_offset: self.position }));
            }
            if inspected == grant { return Ok((inspected, HeaderStep::More)); }
            let byte = source.get(self.position).copied().ok_or("truncated indirect object header")?;
            inspected += 1;
            if self.comment {
                self.comment = !matches!(byte, b'\n' | b'\r');
            } else {
                self.state = match self.state {
                    State::StartNumber if is_ws(byte) => State::StartNumber,
                    State::StartGeneration if is_ws(byte) => State::StartGeneration,
                    State::StartNumber | State::StartGeneration | State::Keyword(0) if byte == b'%' => {
                        self.comment = true;
                        self.state
                    }
                    State::StartNumber | State::Number if byte.is_ascii_digit() => {
                        self.number = self.number.checked_mul(10).and_then(|n| n.checked_add(u32::from(byte - b'0'))).ok_or("bad object number")?;
                        State::Number
                    }
                    State::Number if is_ws(byte) || byte == b'%' => {
                        self.comment = byte == b'%';
                        State::StartGeneration
                    }
                    State::StartGeneration | State::Generation if byte.is_ascii_digit() => {
                        self.generation = self.generation.checked_mul(10).and_then(|n| n.checked_add(u16::from(byte - b'0'))).ok_or("bad generation number")?;
                        State::Generation
                    }
                    State::Generation if is_ws(byte) || byte == b'%' => {
                        self.comment = byte == b'%';
                        State::Keyword(0)
                    }
                    State::Keyword(0) if is_ws(byte) => State::Keyword(0),
                    State::Keyword(3) if is_ws(byte) || is_delim(byte) => {
                        self.state = State::Complete;
                        continue;
                    }
                    State::Keyword(index) if b"obj".get(index) == Some(&byte) => State::Keyword(index + 1),
                    _ => return Err("invalid indirect object header"),
                };
            }
            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genuine_indirect_object_header_resumes_after_every_source_byte() {
        let pdf = include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️indirect-length.pdf");
        let start = pdf.windows(7).position(|window| window == b"4 0 obj").unwrap();
        for grant in 1..=12 {
            let mut cursor = IndirectHeaderCursor::new(start);
            loop {
                let (inspected, step) = cursor.advance(pdf, grant).unwrap();
                assert!(inspected <= grant);
                if let HeaderStep::Complete { id, body_offset } = step {
                    assert_eq!(id, ObjRef { num: 4, gen: 0 });
                    assert_eq!(body_offset, start + 7);
                    assert!(pdf[body_offset..].starts_with(b"\n<< /Length 5 0 R"));
                    break;
                }
            }
        }
    }

    #[test]
    fn comments_and_delimiters_do_not_change_numbers() {
        let source = b"% lead\r\n7% object\n65535% generation\robj<<";
        let mut cursor = IndirectHeaderCursor::new(0);
        let (_, complete) = cursor.advance(source, source.len()).unwrap();
        assert_eq!(complete, HeaderStep::Complete { id: ObjRef { num: 7, gen: 65535 }, body_offset: source.len() - 2 });
        assert!(IndirectHeaderCursor::new(0).advance(b"7 65536 obj ", 64).is_err());
        assert!(IndirectHeaderCursor::new(0).advance(b"7 0 object ", 64).is_err());
    }
}
