//! 🖼️ Resumes PDF inline image boundary validation without copying source bytes.

use super::{is_ws, StreamSpan};

#[derive(Clone, Copy)]
pub enum InlineImageBoundary { Packed(usize), Ascii85, Delimited }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlineImageSpanStep { More, Complete(StreamSpan), Cancelled }

enum Phase { Start, Ascii85, Delimited, Separator, Suffix, Complete(StreamSpan), Failed(&'static str), Cancelled }

/// 📍 Preserves a packed body's exact extent and recognizes ASCII85's own encoded end marker.
pub struct InlineImageSpanCursor {
    boundary: InlineImageBoundary,
    position: usize,
    start: usize,
    end: usize,
    phase: Phase,
}

impl InlineImageSpanCursor {
    pub fn new(id_end: usize, boundary: InlineImageBoundary) -> Self {
        Self { boundary, position: id_end, start: 0, end: 0, phase: Phase::Start }
    }

    /// ▶️ Pays one frame transition per unit and keeps its source immutable across turns.
    pub fn advance(&mut self, data: &[u8], grant: usize, cancelled: bool) -> (usize, Result<InlineImageSpanStep, &'static str>) {
        use InlineImageSpanStep as Step;

        if cancelled && !matches!(self.phase, Phase::Complete(_) | Phase::Failed(_)) { self.phase = Phase::Cancelled; }
        let mut work = 0;
        loop {
            match self.phase {
                Phase::Complete(span) => return (work, Ok(Step::Complete(span))),
                Phase::Failed(reason) => return (work, Err(reason)),
                Phase::Cancelled => return (work, Ok(Step::Cancelled)),
                _ if work == grant => return (work, Ok(Step::More)),
                _ => {}
            }
            self.scan(data);
            work += 1;
        }
    }

    fn scan(&mut self, data: &[u8]) {
        let tail = data.get(self.position..).unwrap_or_default();
        match self.phase {
            Phase::Start => {
                let prefix = match tail {
                    [b'\r', b'\n', ..] => 2,
                    [separator, ..] if is_ws(*separator) => 1,
                    _ => { self.phase = Phase::Failed("missing separator after ID"); return; }
                };
                self.start = self.position + prefix;
                self.position = self.start;
                self.phase = match self.boundary {
                    InlineImageBoundary::Packed(length) => {
                        let Some(end) = self.start.checked_add(length).filter(|&end| end < data.len()) else {
                            self.phase = Phase::Failed("inline image body is shorter than its declared byte length");
                            return;
                        };
                        self.end = end;
                        self.position = end;
                        Phase::Separator
                    }
                    InlineImageBoundary::Ascii85 => Phase::Ascii85,
                    InlineImageBoundary::Delimited => {
                        self.position += 1;
                        Phase::Delimited
                    }
                };
            }
            Phase::Ascii85 => {
                if tail.starts_with(b"~>") {
                    self.position += 2;
                    self.end = self.position;
                    self.phase = Phase::Separator;
                } else if !tail.is_empty() {
                    self.position += 1;
                } else {
                    self.phase = Phase::Failed("missing end marker for ASCII85 inline image");
                }
            }
            Phase::Delimited => {
                if tail.is_empty() {
                    self.phase = Phase::Failed("missing closing EI for filtered inline image");
                } else if is_ws(data[self.position - 1]) && closes(tail) {
                    self.end = self.position - 1;
                    self.complete();
                } else {
                    self.position += 1;
                }
            }
            Phase::Separator => {
                self.phase = if tail.first().copied().is_some_and(is_ws) {
                    self.position += 1;
                    Phase::Suffix
                } else {
                    Phase::Failed(match self.boundary {
                        InlineImageBoundary::Packed(_) => "missing separator before EI at declared byte length",
                        _ => "missing separator before EI after ASCII85 end marker",
                    })
                };
            }
            Phase::Suffix => {
                if tail.first().copied().is_some_and(is_ws) {
                    self.position += 1;
                } else if closes(tail) {
                    self.complete();
                } else {
                    self.phase = Phase::Failed(match self.boundary {
                        InlineImageBoundary::Packed(_) => "expected closing EI at declared byte length",
                        _ => "expected closing EI after ASCII85 end marker",
                    });
                }
            }
            _ => unreachable!("inline image cursor cannot scan in a terminal phase"),
        }
    }

    fn complete(&mut self) {
        self.phase = Phase::Complete(StreamSpan { start: self.start, end: self.end, resume: self.position + 2 });
    }
}

fn closes(tail: &[u8]) -> bool {
    tail.starts_with(b"EI") && tail.get(2).copied().is_none_or(is_ws)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii85_end_marker_owns_the_inline_boundary_under_single_work_grants() {
        let input = b" !! EI !~>\nEI\nQ";
        let mut cursor = InlineImageSpanCursor::new(0, InlineImageBoundary::Ascii85);
        for _ in 0..100 {
            let (work, step) = cursor.advance(input, 1, false);
            assert!(work <= 1);
            if let InlineImageSpanStep::Complete(span) = step.unwrap() {
                assert_eq!(&input[span.start..span.end], b"!! EI !~>");
                assert_eq!(&input[span.resume..], b"\nQ");
                return;
            }
        }
        panic!("inline ASCII85 span did not settle");
    }
}
