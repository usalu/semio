//! 🔤️ Bounded PDF literal and hexadecimal string decoding.

#[derive(Clone, Copy)]
enum State {
    Opening,
    Literal,
    Escape,
    Octal { value: u8, digits: u8 },
    AfterCr,
    Hex(Option<u8>),
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringStep { More, NeedsOutput, Complete }

pub struct StringReceipt {
    pub status: Result<StringStep, &'static str>,
    pub written: usize,
    pub work: usize,
}

pub struct PdfByteStringCursor {
    position: usize,
    depth: usize,
    pending: Option<u8>,
    state: State,
}

impl PdfByteStringCursor {
    pub fn new(position: usize) -> Self {
        Self { position, depth: 0, pending: None, state: State::Opening }
    }

    pub fn position(&self) -> usize { self.position }

    /// 📥️ Decode into caller-owned storage and account for every transition, including refusals.
    pub fn advance(&mut self, source: &[u8], output: &mut [u8], budget: usize) -> StringReceipt {
        let mut written = 0;
        let mut work = 0;
        let status = loop {
            if let Some(byte) = self.pending {
                if written == output.len() { break Ok(StringStep::NeedsOutput); }
                if work == budget { break Ok(StringStep::More); }
                output[written] = byte;
                written += 1;
                work += 1;
                self.pending = None;
            } else if matches!(self.state, State::Done) {
                break Ok(StringStep::Complete);
            } else {
                if work == budget { break Ok(StringStep::More); }
                work += 1;
                let Some(&byte) = source.get(self.position) else { break Err("unterminated PDF byte string"); };
                match self.read(byte) {
                    Ok(true) => self.position += 1,
                    Ok(false) => {}
                    Err(reason) => break Err(reason),
                }
            }
        };
        StringReceipt { status, written, work }
    }

    fn read(&mut self, byte: u8) -> Result<bool, &'static str> {
        match self.state {
            State::Opening => match byte {
                b'(' => { self.depth = 1; self.state = State::Literal; }
                b'<' => self.state = State::Hex(None),
                _ => return Err("expected a PDF byte string"),
            },
            State::Literal => match byte {
                b'(' => { self.depth += 1; self.pending = Some(byte); }
                b')' => {
                    self.depth -= 1;
                    if self.depth == 0 { self.state = State::Done; } else { self.pending = Some(byte); }
                }
                b'\\' => self.state = State::Escape,
                _ => self.pending = Some(byte),
            },
            State::AfterCr => { self.state = State::Literal; return Ok(byte == b'\n'); }
            State::Escape => {
                self.state = State::Literal;
                match byte {
                    b'\r' => self.state = State::AfterCr,
                    b'\n' => {}
                    b'0'..=b'7' => self.state = State::Octal { value: byte - b'0', digits: 1 },
                    _ => self.pending = Some(match byte {
                        b'n' => b'\n', b'r' => b'\r', b't' => b'\t', b'b' => 8, b'f' => 12, other => other,
                    }),
                }
            }
            State::Octal { value, digits } => {
                if !(b'0'..=b'7').contains(&byte) {
                    self.pending = Some(value);
                    self.state = State::Literal;
                    return Ok(false);
                }
                let value = (value << 3) | (byte - b'0');
                if digits == 2 {
                    self.pending = Some(value);
                    self.state = State::Literal;
                } else {
                    self.state = State::Octal { value, digits: digits + 1 };
                }
            }
            State::Hex(high) => {
                if byte == b'>' {
                    self.pending = high.map(|value| value << 4);
                    self.state = State::Done;
                } else if let Some(nibble) = (byte as char).to_digit(16) {
                    let nibble = nibble as u8;
                    self.state = match high {
                        Some(first) => { self.pending = Some((first << 4) | nibble); State::Hex(None) }
                        None => State::Hex(Some(nibble)),
                    };
                }
            }
            State::Done => return Ok(false),
        }
        Ok(true)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
