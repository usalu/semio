//! 🎫️ Tokenizes page and form programs under byte grants while lending bare operator spans.

use semio_framework_value::ByteReadSource;
use super::{is_delim, is_ws, CosGrant, CosLimits, CosStatus, PdfCosCursor, PdfObject};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentTokenEvent {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Ready,
    Cancelled,
    Closed,
}

pub enum ContentToken {
    Value(PdfObject),
    Operator { start: usize, end: usize },
    End,
}

pub struct ContentTokenReceipt {
    pub event: Result<ContentTokenEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

#[derive(Clone, Copy)]
enum Located { Operand, Word(usize), Boolean(bool), Null, End }
enum Phase { Seek, Comment, Word(usize), Operand, Ready(Located), Retire, Failed(&'static str), Cancelled, Closing, Closed }

/// 🧭️ Holds one partially parsed operand and keeps every operator in its source.
pub struct ContentTokenCursor {
    phase: Phase,
    position: usize,
    operand: PdfCosCursor,
}

impl ContentTokenCursor {
    pub fn new(position: usize) -> Self { Self { phase: Phase::Seek, position, operand: PdfCosCursor::idle() } }
    pub fn position(&self) -> usize { self.position }

    /// 🔃️ Resumes after the separately framed binary body of an inline image.
    pub fn resume_after_image(&mut self, position: usize) -> bool {
        if !matches!(self.phase, Phase::Seek) || position < self.position { return false; }
        self.position = position;
        true
    }

    pub fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), mut grant: CosGrant, cancelled: bool) -> ContentTokenReceipt {
        use ContentTokenEvent as Event;
        if cancelled && !matches!(self.phase, Phase::Closing | Phase::Closed) { self.phase = Phase::Cancelled; }
        let mut result = step(Ok(Event::More), 0);
        loop {
            result.event = match self.phase {
                Phase::Ready(_) => Ok(Event::Ready),
                Phase::Failed(reason) => Err(reason),
                Phase::Cancelled => Ok(Event::Cancelled),
                Phase::Closed => Ok(Event::Closed),
                Phase::Closing => Ok(Event::More),
                _ if grant.work == 0 => Ok(Event::More),
                Phase::Seek | Phase::Comment | Phase::Word(_) => {
                    grant.work -= 1;
                    result.work += 1;
                    self.scan(source);
                    continue;
                }
                Phase::Operand => {
                    let cursor = &mut self.operand;
                    let child = cursor.advance_bytes(|position| source.byte_at(position), grant, false);
                    let status = child.status;
                    result.work += child.work;
                    result.allocated_bytes += child.allocated_bytes;
                    result.released_bytes += child.released_bytes;
                    match status {
                        Ok(CosStatus::Complete { end }) => {
                            self.position = end;
                            self.phase = Phase::Ready(Located::Operand);
                            Ok(Event::Ready)
                        }
                        Err(reason) => {
                            self.position = cursor.inspected_position();
                            self.phase = Phase::Failed(reason);
                            Err(reason)
                        }
                        status => status.map(event_from_cos),
                    }
                }
                Phase::Retire => {
                    let child = self.retire_operand(grant);
                    result.work += child.work;
                    result.released_bytes += child.released_bytes;
                    child.event
                }
            };
            return result;
        }
    }

    fn scan(&mut self, source: &(impl ByteReadSource + ?Sized)) {
        let byte = source.byte_at(self.position);
        match self.phase {
            Phase::Comment => match byte {
                Some(byte) => {
                    self.position += 1;
                    if matches!(byte, b'\r' | b'\n') { self.phase = Phase::Seek; }
                }
                None => self.phase = Phase::Ready(Located::End),
            },
            Phase::Word(start) => {
                if byte.is_some_and(|byte| !is_ws(byte) && !is_delim(byte)) {
                    self.position += 1;
                } else {
                    let mut spelling = [0u8; 5];
                    let length = self.position - start;
                    let word = match (length <= spelling.len() && source.read_bytes(start, &mut spelling[..length]) == length).then(|| &spelling[..length]) {
                        Some(b"true") => Located::Boolean(true),
                        Some(b"false") => Located::Boolean(false),
                        Some(b"null") => Located::Null,
                        _ => Located::Word(start),
                    };
                    self.phase = Phase::Ready(word);
                }
            }
            Phase::Seek => match byte {
                None => self.phase = Phase::Ready(Located::End),
                Some(b'%') => { self.position += 1; self.phase = Phase::Comment; }
                Some(b'/' | b'(' | b'<' | b'[' | b'+' | b'-' | b'.' | b'0'..=b'9') => {
                    self.phase = if self.operand.rearm(self.position, CosLimits::default()) { Phase::Operand } else { Phase::Failed("PDF content operand is still retiring") };
                }
                Some(byte) if is_ws(byte) || is_delim(byte) => self.position += 1,
                Some(_) => {
                    self.phase = Phase::Word(self.position);
                    self.position += 1;
                }
            },
            _ => unreachable!("content token scan requires a lexical phase"),
        }
    }

    /// 🫴️ Transfers an operand or lends the exact bounds of an operator in the original.
    pub fn take(&mut self) -> Option<ContentToken> {
        let Phase::Ready(token) = self.phase else { return None; };
        self.phase = Phase::Seek;
        Some(match token {
            Located::Operand => {
                let cursor = &mut self.operand;
                let value = cursor.take_value()?;
                cursor.begin_close();
                self.phase = Phase::Retire;
                ContentToken::Value(value)
            }
            Located::Word(start) => ContentToken::Operator { start, end: self.position },
            Located::Boolean(value) => ContentToken::Value(PdfObject::Bool(value)),
            Located::Null => ContentToken::Value(PdfObject::Null),
            Located::End => ContentToken::End,
        })
    }

    /// 🛟️ Discards a malformed operand while retaining the remaining original program.
    pub fn recover(&mut self) -> bool {
        if !matches!(self.phase, Phase::Failed(_)) { return false; }
        if self.operand.terminal_is_empty() { return false; }
        self.position = self.position.saturating_add(1);
        self.operand.begin_close();
        self.phase = Phase::Retire;
        true
    }

    /// 🧰️ Drives one token for the reference content reader through the same lexical path.
    pub fn next_token(&mut self, source: &(impl ByteReadSource + ?Sized)) -> Result<ContentToken, &'static str> {
        let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
        loop {
            match self.advance(source, grant, false).event? {
                ContentTokenEvent::Ready => return self.take().ok_or("complete PDF content token is unavailable"),
                ContentTokenEvent::More => {}
                _ => return Err("unexpected PDF content token state"),
            }
        }
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::Closed | Phase::Closing) {
            self.operand.begin_close();
            self.phase = Phase::Closing;
        }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> ContentTokenReceipt {
        use ContentTokenEvent as Event;
        match self.phase {
            Phase::Closed => step(Ok(Event::Closed), 0),
            Phase::Closing if self.operand.terminal_is_empty() => {
                self.phase = Phase::Closed;
                step(Ok(Event::Closed), 0)
            }
            Phase::Closing => self.retire_operand(grant),
            _ => step(Ok(Event::More), 0),
        }
    }

    fn retire_operand(&mut self, grant: CosGrant) -> ContentTokenReceipt {
        let turn = self.operand.close_step(grant);
        if turn.status == Ok(CosStatus::Closed) {
            if matches!(self.phase, Phase::Retire) { self.phase = Phase::Seek; }
        }
        ContentTokenReceipt { event: turn.status.map(event_from_cos), work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes }
    }
}

fn step(event: Result<ContentTokenEvent, &'static str>, work: usize) -> ContentTokenReceipt {
    ContentTokenReceipt { event, work, allocated_bytes: 0, released_bytes: 0 }
}

fn event_from_cos(status: CosStatus) -> ContentTokenEvent {
    match status {
        CosStatus::NeedsAllocation { bytes } => ContentTokenEvent::NeedsAllocation { bytes },
        CosStatus::NeedsRelease { bytes } => ContentTokenEvent::NeedsRelease { bytes },
        _ => ContentTokenEvent::More,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
