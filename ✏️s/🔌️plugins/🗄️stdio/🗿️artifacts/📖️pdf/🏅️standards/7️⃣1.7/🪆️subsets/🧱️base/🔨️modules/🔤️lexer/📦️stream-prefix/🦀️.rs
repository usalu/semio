//! 📦️ Bounded recognition of the stream introducer following a COS dictionary.

use super::is_ws;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamPrefixStep {
    More,
    Complete(Option<usize>),
}

pub struct StreamPrefixCursor {
    position: usize,
    matched: usize,
    comment: bool,
    carriage_return: bool,
    result: Option<Option<usize>>,
}

impl StreamPrefixCursor {
    pub fn new(dictionary_end: usize) -> Self {
        Self { position: dictionary_end, matched: 0, comment: false, carriage_return: false, result: None }
    }

    /// 📏️ After the dictionary, scan the introducer and its CR, LF, or CRLF delimiter.
    pub fn advance(&mut self, source: &[u8], grant: usize) -> (usize, StreamPrefixStep) {
        self.advance_bytes(|position| source.get(position).copied(), grant)
    }

    pub(super) fn advance_bytes(&mut self, byte_at: impl Fn(usize) -> Option<u8>, grant: usize) -> (usize, StreamPrefixStep) {
        let mut work = 0;
        loop {
            if let Some(result) = self.result { return (work, StreamPrefixStep::Complete(result)); }
            if work == grant { return (work, StreamPrefixStep::More); }
            work += 1;
            let byte = byte_at(self.position);
            match byte {
                _ if self.matched == 6 => {
                    match byte {
                        Some(b'\r') if !self.carriage_return => self.carriage_return = true,
                        Some(b'\n') => self.result = Some(Some(self.position + 1)),
                        _ => {
                            self.result = Some(Some(self.position));
                            continue;
                        }
                    }
                }
                Some(byte) if self.comment => self.comment = !matches!(byte, b'\r' | b'\n'),
                Some(b'%') if self.matched == 0 => self.comment = true,
                Some(byte) if self.matched == 0 && is_ws(byte) => {}
                Some(byte) if b"stream"[self.matched] == byte => self.matched += 1,
                _ => {
                    self.result = Some(None);
                    continue;
                }
            }
            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_prefix_resumes_across_comments_keyword_and_line_endings() {
        for (source, expected) in [
            (b">> % hello\r\nstream\r\nbody".as_slice(), Some(20)),
            (b">>stream\nbody", Some(9)),
            (b">>stream\rbody", Some(9)),
            (b">>  endobj", None),
        ] {
            let mut cursor = StreamPrefixCursor::new(2);
            loop {
                let (work, step) = cursor.advance(source, 1);
                assert!(work <= 1);
                if let StreamPrefixStep::Complete(body) = step {
                    assert_eq!(body, expected);
                    break;
                }
            }
        }
    }
}
