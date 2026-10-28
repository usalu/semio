//! 🧭 Borrowed original-byte lexical scans for IFC STEP framing and source locations.
use super::{checkpoint, refusal, IfcStepError, IfcStepProgress};

pub(super) struct SourceWalk<'a> {
    pub bytes: &'a [u8],
    pub at: usize,
    base: usize,
    total: usize,
    limit: usize,
    checked: usize,
}

impl<'a> SourceWalk<'a> {
    pub fn new(text: &'a str, at: usize, base: usize, total: usize, limit: usize) -> Self { Self { bytes: text.as_bytes(), at, base, total, limit, checked: at } }

    fn visit(&mut self, at: usize, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<(), IfcStepError> {
        if at >= self.limit && at < self.bytes.len() { return Err(refusal("step.record-limit", "STEP record exceeds the bounded source unit", self.base + at)); }
        if at.saturating_sub(self.checked) >= 4096 { self.checked = at; checkpoint(control, "scan", self.base + at, self.total)?; }
        Ok(())
    }

    fn next_at(&mut self, mut at: usize, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<Option<(usize, u8)>, IfcStepError> {
        while let Some(&byte) = self.bytes.get(at) { self.visit(at, control)?; if byte < 32 || byte == 127 { at += 1; } else { return Ok(Some((at, byte))); } }
        Ok(None)
    }

    pub fn peek(&mut self, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<Option<u8>, IfcStepError> {
        if let Some((at, byte)) = self.next_at(self.at, control)? { self.at = at; Ok(Some(byte)) } else { self.at = self.bytes.len(); Ok(None) }
    }

    pub fn prefix(&mut self, expected: &[u8], control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<Option<usize>, IfcStepError> {
        let mut at = self.at;
        for &expected in expected { let Some((position, byte)) = self.next_at(at, control)? else { return Ok(None); }; if byte != expected { return Ok(None); } at = position + 1; }
        Ok(Some(at))
    }

    pub fn consume(&mut self, expected: &[u8], control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<bool, IfcStepError> {
        if let Some(at) = self.prefix(expected, control)? { self.at = at; Ok(true) } else { Ok(false) }
    }

    pub fn trivia(&mut self, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<(), IfcStepError> {
        loop {
            if self.peek(control)? == Some(b' ') { self.at += 1; }
            else if self.consume(b"\\N\\", control)? || self.consume(b"\\F\\", control)? { }
            else if self.consume(b"/*", control)? {
                let start = self.at;
                loop { if self.consume(b"*/", control)? { break; } if self.peek(control)?.is_none() { return Err(refusal("step.comment", "Unterminated STEP comment", self.base + start)); } self.at += 1; }
            } else { return Ok(()); }
        }
    }

    pub fn quoted(&mut self, quote: u8, control: &mut dyn FnMut(IfcStepProgress) -> bool) -> Result<(), IfcStepError> {
        let start = self.at;
        self.at += 1;
        loop {
            let Some(byte) = self.peek(control)? else { return Err(refusal("step.string", "Unterminated STEP string or binary literal", self.base + start)); };
            if quote == b'\'' && byte == b'\\' {
                if self.consume(b"\\\\", control)? { continue; }
                if self.consume(b"\\S\\", control)? { if self.peek(control)?.is_none() { return Err(refusal("step.string", "Missing PAGE payload", self.base + self.at)); } self.at += 1; continue; }
            }
            self.at += 1;
            if byte == quote { if quote == b'\'' && self.peek(control)? == Some(quote) { self.at += 1; } else { return Ok(()); } }
        }
    }
}
