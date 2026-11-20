//! 📏️ Resumable stream boundary validation over immutable PDF source bytes.

const MARKER: &[u8] = b"endstream";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamSpan {
    pub start: usize,
    pub end: usize,
    pub resume: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamSpanStep {
    pub inspected: usize,
    pub span: Option<StreamSpan>,
}

#[derive(Clone, Copy)]
enum Phase {
    Declared { end: usize, comment: bool },
    Search,
    Complete(StreamSpan),
}

pub struct StreamSpanCursor {
    start: usize,
    position: usize,
    matched: usize,
    line_break: usize,
    candidate_trim: usize,
    previous: Option<u8>,
    phase: Phase,
}

impl StreamSpanCursor {
    pub fn new(start: usize, length: Option<usize>, source_len: usize) -> Self {
        let start = start.min(source_len);
        let end = length.and_then(|length| start.checked_add(length)).filter(|&end| end <= source_len);
        let (position, phase) = match end {
            Some(end) => (end, Phase::Declared { end, comment: false }),
            None => (start, Phase::Search),
        };
        Self { start, position, matched: 0, line_break: 0, candidate_trim: 0, previous: None, phase }
    }

    /// 📥️ Inspect at most the granted source bytes; a complete span can be borrowed without allocation.
    pub fn advance(&mut self, source: &[u8], grant: usize) -> StreamSpanStep {
        let mut inspected = 0;
        loop {
            if let Phase::Complete(span) = self.phase {
                return StreamSpanStep { inspected, span: Some(span) };
            }
            if self.position == source.len() {
                if self.matched == MARKER.len() {
                    self.finish_marker();
                } else if matches!(self.phase, Phase::Declared { .. }) {
                    self.start_search();
                } else {
                    self.phase = Phase::Complete(StreamSpan { start: self.start, end: source.len(), resume: source.len() });
                }
                continue;
            }
            if inspected == grant {
                return StreamSpanStep { inspected, span: None };
            }
            let byte = source[self.position];
            inspected += 1;
            if self.matched == MARKER.len() {
                if super::is_ws(byte) || super::is_delim(byte) {
                    self.finish_marker();
                    continue;
                }
                if matches!(self.phase, Phase::Declared { .. }) {
                    self.start_search();
                    continue;
                }
                self.matched = 0;
            }

            match self.phase {
                Phase::Declared { end, comment } if self.matched == 0 && (comment || byte == b'%' || super::is_ws(byte)) => {
                    self.phase = Phase::Declared { end, comment: (comment || byte == b'%') && !matches!(byte, b'\r' | b'\n') };
                }
                Phase::Declared { .. } if byte != MARKER[self.matched] => {
                    self.start_search();
                    continue;
                }
                Phase::Declared { .. } => self.matched += 1,
                Phase::Search if self.matched > 0 && byte == MARKER[self.matched] => self.matched += 1,
                Phase::Search => {
                    let at_line_start = self.position == self.start || self.line_break != 0;
                    self.matched = usize::from(at_line_start && byte == MARKER[0]);
                    if self.matched == 1 { self.candidate_trim = self.line_break; }
                }
                Phase::Complete(_) => unreachable!(),
            }
            self.line_break = match byte {
                b'\r' => 1,
                b'\n' => 1 + usize::from(self.previous == Some(b'\r')),
                _ => 0,
            };
            self.previous = Some(byte);
            self.position += 1;
        }
    }

    fn start_search(&mut self) {
        self.phase = Phase::Search;
        self.position = self.start;
        self.matched = 0;
        self.line_break = 0;
        self.previous = None;
    }

    fn finish_marker(&mut self) {
        let end = match self.phase {
            Phase::Declared { end, .. } => end,
            Phase::Search => self.position - MARKER.len() - self.candidate_trim,
            Phase::Complete(_) => unreachable!(),
        };
        self.phase = Phase::Complete(StreamSpan { start: self.start, end, resume: self.position });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_spans_keep_marker_text_inside_the_payload() {
        let pdf = include_bytes!("../../../🧪️tests/🧫️fixtures/🔗️indirect-length.pdf");
        let start = pdf.windows(8).position(|window| window == b"\nstream\n").unwrap() + 8;
        let payload = b"BT /F1 12 Tf 10 70 Td (prefix endstream suffix) Tj ET";
        for length in [None, Some(payload.len()), Some(0), Some(usize::MAX)] {
            for grant in [1, 2, 7, 64] {
                let mut cursor = StreamSpanCursor::new(start, length, pdf.len());
                let span = loop {
                    let step = cursor.advance(pdf, grant);
                    assert!(step.inspected <= grant);
                    if let Some(span) = step.span { break span; }
                    assert_eq!(step.inspected, grant);
                };
                assert_eq!(&pdf[span.start..span.end], payload);
                assert!(pdf[span.resume..].starts_with(b"\nendobj"));
            }
        }
    }

    #[test]
    fn known_length_wins_over_binary_marker_and_missing_marker_keeps_bytes() {
        let source = b"\x00\nendstream\n\x01\nendstream";
        let span = StreamSpanCursor::new(0, Some(13), source.len()).advance(source, 64).span.unwrap();
        assert_eq!(&source[..span.end], b"\x00\nendstream\n\x01");
        assert_eq!(span.resume, source.len());
        let source = b"data";
        let span = StreamSpanCursor::new(0, None, source.len()).advance(source, 64).span.unwrap();
        assert_eq!(span, StreamSpan { start: 0, end: 4, resume: 4 });
    }
}
