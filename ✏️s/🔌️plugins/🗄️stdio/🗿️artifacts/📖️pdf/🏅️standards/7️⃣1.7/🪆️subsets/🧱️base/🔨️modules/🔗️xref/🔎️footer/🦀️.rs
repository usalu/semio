//! 🔎️ Budgeted discovery of the last startxref pointer in a PDF.

#[derive(Clone, Copy)]
enum Phase {
    Search { matched: usize },
    LeadingBoundary,
    TrailingBoundary,
    Space,
    Comment,
    Digits,
    Done(Option<usize>),
}

pub struct FooterCursor {
    position: usize,
    marker: usize,
    length: usize,
    phase: Phase,
    offset: usize,
    digits: bool,
}

pub enum FooterStep {
    More,
    Complete(Option<usize>),
}

impl FooterCursor {
    pub fn new(length: usize) -> Self {
        Self { position: length, marker: 0, length, phase: Phase::Search { matched: 0 }, offset: 0, digits: false }
    }

    /// 🧭 Inspect at most one input byte per unit of the supplied grant.
    pub fn advance(&mut self, byte_at: impl Fn(usize) -> Option<u8>, budget: usize) -> FooterStep {
        self.advance_counted(byte_at, budget).1
    }

    /// 🧮️ Measures actual footer transitions when composing the probe with other cursors.
    pub(crate) fn advance_counted(&mut self, byte_at: impl Fn(usize) -> Option<u8>, budget: usize) -> (usize, FooterStep) {
        const MARKER: &[u8] = b"ferxtrats";
        let mut work = 0;
        while work < budget && !matches!(self.phase, Phase::Done(_)) {
            work += 1;
            match self.phase {
                Phase::Search { matched } => {
                    if self.position == 0 {
                        self.phase = Phase::Done(None);
                        break;
                    }
                    self.position -= 1;
                    let Some(byte) = byte_at(self.position) else {
                        self.phase = Phase::Done(None);
                        break;
                    };
                    let count = if byte == MARKER[matched] { matched + 1 } else { usize::from(byte == MARKER[0]) };
                    if count == MARKER.len() {
                        self.marker = self.position;
                        self.phase = Phase::LeadingBoundary;
                    } else {
                        self.phase = Phase::Search { matched: count };
                    }
                }
                Phase::LeadingBoundary => {
                    if self.marker == 0 || byte_at(self.marker - 1).is_some_and(whitespace) {
                        self.position = self.marker + MARKER.len();
                        self.phase = Phase::TrailingBoundary;
                    } else { self.earlier_candidate(); }
                }
                Phase::TrailingBoundary => {
                    if byte_at(self.position).is_some_and(|byte| whitespace(byte) || byte == b'%') {
                        self.phase = Phase::Space;
                    } else { self.earlier_candidate(); }
                }
                Phase::Space | Phase::Comment | Phase::Digits => {
                    let Some(byte) = byte_at(self.position) else {
                        self.settle(self.length, true);
                        continue;
                    };
                    self.position += 1;
                    let space = whitespace(byte);
                    match self.phase {
                        Phase::Comment => {
                            if matches!(byte, b'\r' | b'\n') { self.phase = Phase::Space; }
                        }
                        Phase::Space if space => {}
                        Phase::Space if byte == b'%' => self.phase = Phase::Comment,
                        Phase::Space if byte == b'+' => self.phase = Phase::Digits,
                        _ if byte.is_ascii_digit() => {
                            let Some(next) = self.offset.checked_mul(10).and_then(|n| n.checked_add(usize::from(byte - b'0'))) else {
                                self.earlier_candidate();
                                continue;
                            };
                            self.offset = next;
                            self.digits = true;
                            self.phase = Phase::Digits;
                        }
                        _ => {
                            let delimiter = matches!(byte, b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%');
                            self.settle(self.length, space || delimiter);
                        }
                    }
                }
                Phase::Done(_) => break,
            }
        }
        let stage = match self.phase {
            Phase::Done(offset) => FooterStep::Complete(offset),
            _ => FooterStep::More,
        };
        (work, stage)
    }

    fn earlier_candidate(&mut self) {
        self.position = self.marker;
        self.phase = Phase::Search { matched: 0 };
        self.offset = 0;
        self.digits = false;
    }

    fn settle(&mut self, length: usize, delimited: bool) {
        if delimited && self.digits && self.offset < length {
            self.phase = Phase::Done(Some(self.offset));
        } else { self.earlier_candidate(); }
    }
}

fn whitespace(byte: u8) -> bool { matches!(byte, 0 | 9 | 10 | 12 | 13 | 32) }

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
