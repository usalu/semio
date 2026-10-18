//! 🔡️ Path and token cursor: the schema path, the token kinds, the lexer states and the token cursor. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaPath {
    pub(super) bytes: [u8; 256],
    pub(super) len: u8,
}

impl OwnedSchemaPath {
    pub const ROOT: Self = {
        let mut bytes = [0; 256];
        bytes[0] = b'$';
        Self { bytes, len: 1 }
    };

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..self.len as usize]) }
    }

    pub fn field(key: &'static str) -> Option<Self> {
        let len = key.len().checked_add(2)?;
        if len > u8::MAX as usize || !key.is_ascii() {
            return None;
        }
        let mut bytes = [0; 256];
        bytes[0] = b'$';
        bytes[1] = b'.';
        bytes[2..len].copy_from_slice(key.as_bytes());
        Some(Self { bytes, len: len as u8 })
    }
}

pub type OwnedSchemaDecodeDiagnostic=schema_diagnostic::SchemaDecodeDiagnostic<OwnedSchemaPath>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnedSchemaTokenKind {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Colon,
    Comma,
    String,
    Number,
    True,
    False,
    Null,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaToken {
    pub kind: OwnedSchemaTokenKind,
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Copy)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaLexState {
    Ready,
    String { start: usize, escape: bool, unicode: u8, utf8: u8, next_min: u8, next_max: u8 },
    Number { start: usize, bytes: [u8; 64], len: u8 },
    Literal { start: usize, kind: OwnedSchemaTokenKind, expected: &'static [u8], index: u8 },
    Complete,
    Fault(OwnedSchemaDecodeDiagnostic),
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaTokenStep {
    Pending,
    Token(OwnedSchemaToken),
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

pub struct OwnedSchemaTokenCursor {
    pub(crate) pages: OwnedSchemaDecodePages,
    pub(super) state: OwnedSchemaLexState,
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) offset: usize,
    pub(super) line: u32,
    pub(super) column: u32,
}

impl OwnedSchemaTokenCursor {
    /// 🧳️ Returns unchanged pages after schema admission refuses the original cursor.
    pub fn into_pages(self) -> OwnedSchemaDecodePages { self.pages }

    /// 🧬️ Begins tokenization only after exact fixed-page admission is sealed.
    pub fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pages: OwnedSchemaDecodePages) -> Result<Self, OwnedSchemaDecodePages> {
        if !pages.is_sealed() {
            return Err(pages);
        }
        Ok(Self { pages, state: OwnedSchemaLexState::Ready, operation, generation, offset: 0, line: 1, column: 1 })
    }

    pub(super) fn diagnostic(&self, code: &'static str, offset: usize) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: offset as u64, line: self.line, column: self.column, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn advance(&mut self, byte: u8) {
        self.offset += 1;
        if byte == b'\n' {
            self.line = self.line.saturating_add(1);
            self.column = 1;
        } else {
            self.column = self.column.saturating_add(1);
        }
    }

    fn punct(&mut self, byte: u8, kind: OwnedSchemaTokenKind, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep {
        let start = self.offset;
        self.advance(byte);
        cx.consume_fuel(1);
        OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind, start: start as u64, end: self.offset as u64 })
    }

    fn valid_number(bytes: &[u8]) -> bool {
        let mut cursor = 0;
        if bytes.get(cursor) == Some(&b'-') {
            cursor += 1;
        }
        match bytes.get(cursor) {
            Some(b'0') => cursor += 1,
            Some(b'1'..=b'9') => {
                cursor += 1;
                while matches!(bytes.get(cursor), Some(b'0'..=b'9')) {
                    cursor += 1;
                }
            }
            _ => return false,
        }
        if bytes.get(cursor) == Some(&b'.') {
            cursor += 1;
            let fraction = cursor;
            while matches!(bytes.get(cursor), Some(b'0'..=b'9')) {
                cursor += 1;
            }
            if cursor == fraction {
                return false;
            }
        }
        if matches!(bytes.get(cursor), Some(b'e' | b'E')) {
            cursor += 1;
            if matches!(bytes.get(cursor), Some(b'+' | b'-')) {
                cursor += 1;
            }
            let exponent = cursor;
            while matches!(bytes.get(cursor), Some(b'0'..=b'9')) {
                cursor += 1;
            }
            if cursor == exponent {
                return false;
            }
        }
        cursor == bytes.len()
    }

    fn finish_eof(&mut self) -> OwnedSchemaTokenStep {
        match self.state {
            OwnedSchemaLexState::Ready => {
                self.state = OwnedSchemaLexState::Complete;
                OwnedSchemaTokenStep::Complete
            }
            OwnedSchemaLexState::Number { start, bytes, len } if Self::valid_number(&bytes[..len as usize]) => {
                self.state = OwnedSchemaLexState::Ready;
                OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind: OwnedSchemaTokenKind::Number, start: start as u64, end: self.offset as u64 })
            }
            OwnedSchemaLexState::Complete => OwnedSchemaTokenStep::Complete,
            OwnedSchemaLexState::Cancelled => OwnedSchemaTokenStep::Cancelled,
            OwnedSchemaLexState::Fault(diagnostic) => OwnedSchemaTokenStep::Fault(diagnostic),
            _ => {
                let diagnostic = self.diagnostic("schema-json.truncated-token", self.offset);
                self.state = OwnedSchemaLexState::Fault(diagnostic);
                OwnedSchemaTokenStep::Fault(diagnostic)
            }
        }
    }

    /// ⏭️ Advances lexical validation by budgeted bytes and yields at most one schema token.
    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            let diagnostic = self.diagnostic("schema-json.stale-authority", self.offset);
            self.state = OwnedSchemaLexState::Fault(diagnostic);
            return OwnedSchemaTokenStep::Fault(diagnostic);
        }
        if cx.is_cancelled() {
            self.state = OwnedSchemaLexState::Cancelled;
            return OwnedSchemaTokenStep::Cancelled;
        }
        cx.set_stage("schema-json-token");
        loop {
            if cx.should_yield() {
                return OwnedSchemaTokenStep::Pending;
            }
            let Some(byte) = self.pages.byte_at(self.offset) else {
                return self.finish_eof();
            };
            let state = self.state;
            let outcome = match state {
                OwnedSchemaLexState::Ready => match byte {
                    b' ' | b'\t' | b'\r' | b'\n' => {
                        self.advance(byte);
                        None
                    }
                    b'{' => return self.punct(byte, OwnedSchemaTokenKind::ObjectStart, cx),
                    b'}' => return self.punct(byte, OwnedSchemaTokenKind::ObjectEnd, cx),
                    b'[' => return self.punct(byte, OwnedSchemaTokenKind::ArrayStart, cx),
                    b']' => return self.punct(byte, OwnedSchemaTokenKind::ArrayEnd, cx),
                    b':' => return self.punct(byte, OwnedSchemaTokenKind::Colon, cx),
                    b',' => return self.punct(byte, OwnedSchemaTokenKind::Comma, cx),
                    b'"' => {
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::String { start, escape: false, unicode: 0, utf8: 0, next_min: 0x80, next_max: 0xbf };
                        None
                    }
                    b'-' | b'0'..=b'9' => {
                        let mut bytes = [0; 64];
                        bytes[0] = byte;
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::Number { start, bytes, len: 1 };
                        None
                    }
                    b't' => {
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::Literal { start, kind: OwnedSchemaTokenKind::True, expected: b"true", index: 1 };
                        None
                    }
                    b'f' => {
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::Literal { start, kind: OwnedSchemaTokenKind::False, expected: b"false", index: 1 };
                        None
                    }
                    b'n' => {
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::Literal { start, kind: OwnedSchemaTokenKind::Null, expected: b"null", index: 1 };
                        None
                    }
                    _ => Some(self.diagnostic("schema-json.unexpected-byte", self.offset)),
                },
                OwnedSchemaLexState::String { start, mut escape, mut unicode, mut utf8, mut next_min, mut next_max } => {
                    if unicode != 0 {
                        if !byte.is_ascii_hexdigit() {
                            Some(self.diagnostic("schema-json.invalid-unicode-escape", self.offset))
                        } else {
                            unicode -= 1;
                            self.advance(byte);
                            self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8, next_min, next_max };
                            None
                        }
                    } else if escape {
                        if !matches!(byte, b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' | b'u') {
                            Some(self.diagnostic("schema-json.invalid-escape", self.offset))
                        } else {
                            escape = false;
                            unicode = usize::from(byte == b'u') as u8 * 4;
                            self.advance(byte);
                            self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8, next_min, next_max };
                            None
                        }
                    } else if utf8 != 0 {
                        if byte < next_min || byte > next_max {
                            Some(self.diagnostic("schema-json.invalid-utf8", self.offset))
                        } else {
                            utf8 -= 1;
                            next_min = 0x80;
                            next_max = 0xbf;
                            self.advance(byte);
                            self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8, next_min, next_max };
                            None
                        }
                    } else {
                        match byte {
                            b'"' => {
                                self.advance(byte);
                                cx.consume_fuel(1);
                                self.state = OwnedSchemaLexState::Ready;
                                return OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind: OwnedSchemaTokenKind::String, start: start as u64, end: self.offset as u64 });
                            }
                            b'\\' => {
                                escape = true;
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8, next_min, next_max };
                                None
                            }
                            0x00..=0x1f => Some(self.diagnostic("schema-json.control-in-string", self.offset)),
                            0x20..=0x7f => {
                                self.advance(byte);
                                None
                            }
                            0xc2..=0xdf => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 1, next_min: 0x80, next_max: 0xbf };
                                None
                            }
                            0xe0 => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 2, next_min: 0xa0, next_max: 0xbf };
                                None
                            }
                            0xe1..=0xec | 0xee..=0xef => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 2, next_min: 0x80, next_max: 0xbf };
                                None
                            }
                            0xed => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 2, next_min: 0x80, next_max: 0x9f };
                                None
                            }
                            0xf0 => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 3, next_min: 0x90, next_max: 0xbf };
                                None
                            }
                            0xf1..=0xf3 => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 3, next_min: 0x80, next_max: 0xbf };
                                None
                            }
                            0xf4 => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 3, next_min: 0x80, next_max: 0x8f };
                                None
                            }
                            _ => Some(self.diagnostic("schema-json.invalid-utf8", self.offset)),
                        }
                    }
                }
                OwnedSchemaLexState::Number { start, mut bytes, mut len } => {
                    if matches!(byte, b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') {
                        if len as usize == bytes.len() {
                            Some(self.diagnostic("schema-json.number-token-too-long", self.offset))
                        } else {
                            bytes[len as usize] = byte;
                            len += 1;
                            self.advance(byte);
                            self.state = OwnedSchemaLexState::Number { start, bytes, len };
                            None
                        }
                    } else if Self::valid_number(&bytes[..len as usize]) {
                        self.state = OwnedSchemaLexState::Ready;
                        return OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind: OwnedSchemaTokenKind::Number, start: start as u64, end: self.offset as u64 });
                    } else {
                        Some(self.diagnostic("schema-json.invalid-number", start))
                    }
                }
                OwnedSchemaLexState::Literal { start, kind, expected, index } => {
                    if expected.get(index as usize) != Some(&byte) {
                        Some(self.diagnostic("schema-json.invalid-literal", self.offset))
                    } else {
                        self.advance(byte);
                        let index = index + 1;
                        if index as usize == expected.len() {
                            cx.consume_fuel(1);
                            self.state = OwnedSchemaLexState::Ready;
                            return OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind, start: start as u64, end: self.offset as u64 });
                        }
                        self.state = OwnedSchemaLexState::Literal { start, kind, expected, index };
                        None
                    }
                }
                OwnedSchemaLexState::Complete => return OwnedSchemaTokenStep::Complete,
                OwnedSchemaLexState::Fault(diagnostic) => return OwnedSchemaTokenStep::Fault(diagnostic),
                OwnedSchemaLexState::Cancelled => return OwnedSchemaTokenStep::Cancelled,
            };
            cx.consume_fuel(1);
            if let Some(diagnostic) = outcome {
                self.state = OwnedSchemaLexState::Fault(diagnostic);
                return OwnedSchemaTokenStep::Fault(diagnostic);
            }
            if cx.is_cancelled() {
                self.state = OwnedSchemaLexState::Cancelled;
                return OwnedSchemaTokenStep::Cancelled;
            }
        }
    }

    /// 🧮️ Names the inline payload work of the next admitted page without retiring it.
    pub fn next_close_copy_byte_demand(&self) -> usize {
        self.pages.close_page_bytes().unwrap_or(0)
    }

    /// 🧹️ Retires exactly one admitted page from a cancelled or rejected decode, charging its inline bytes as payload work.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty=RetainedCloneProgress::default();
        let Some(bytes)=self.pages.close_page_bytes() else { return self.pages.close_backing_step(grant); };
        if grant.maximum_items==0 || grant.maximum_copy_bytes<bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"decode page requires supplied depth")); }
        let page=self.pages.close_take_page().expect("observed page remains retained");
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,copied_bytes:page.len(),..empty }))
    }
    pub fn next_close_capacity_byte_demand(&self, _body:usize)->Result<usize,ValueError>{Ok(0)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.pages.close_page_bytes().is_some(){0}else{self.pages.allocation_byte_demand()})}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
    pub fn terminal_is_empty(&self) -> bool { self.pages.terminal_is_empty() }

    /// 📐️ Copies a bounded token span directly from fixed pages without concatenation.
    pub fn copy_token_bytes(&self, token: OwnedSchemaToken, relative_offset: usize, output: &mut [u8]) -> usize {
        let start = token.start as usize;
        let end = token.end as usize;
        let Some(mut offset) = start.checked_add(relative_offset).filter(|offset| *offset < end) else {
            return 0;
        };
        let maximum = output.len().min(OWNED_SCHEMA_DECODE_PAGE_BYTES);
        let mut written = 0;
        while written < maximum && offset < end {
            let Some(byte) = self.pages.byte_at(offset) else { break };
            output[written] = byte;
            written += 1;
            offset += 1;
        }
        written
    }

    pub(super) fn string_token_equals(&self, token: OwnedSchemaToken, expected: &str) -> bool {
        if token.kind != OwnedSchemaTokenKind::String || !expected.is_ascii() || expected.len() > 253 {
            return false;
        }
        let (Ok(start), Ok(end)) = (usize::try_from(token.start), usize::try_from(token.end)) else { return false };
        let Some(length) = end.checked_sub(start).filter(|length| *length >= 2 && *length <= expected.len() * 6 + 2) else { return false };
        if self.pages.byte_at(start) != Some(b'"') || self.pages.byte_at(start + length - 1) != Some(b'"') {
            return false;
        }
        let read = |offset| if offset < end - 1 { self.pages.byte_at(offset) } else { None };
        let mut offset = start + 1;
        for expected in expected.bytes() {
            let Some(mut byte) = read(offset) else { return false };
            offset += 1;
            if byte == b'\\' {
                let Some(escape) = read(offset) else { return false };
                offset += 1;
                byte = match escape {
                    b'"' | b'\\' | b'/' => escape,
                    b'b' => 8,
                    b'f' => 12,
                    b'n' => b'\n',
                    b'r' => b'\r',
                    b't' => b'\t',
                    b'u' => {
                        let mut value = 0u16;
                        for _ in 0..4 {
                            let Some(digit) = read(offset) else { return false };
                            let digit = match digit {
                                b'0'..=b'9' => digit - b'0',
                                b'a'..=b'f' => digit - b'a' + 10,
                                b'A'..=b'F' => digit - b'A' + 10,
                                _ => return false,
                            };
                            value = value * 16 + u16::from(digit);
                            offset += 1;
                        }
                        if value > 127 {
                            return false;
                        }
                        value as u8
                    }
                    _ => return false,
                };
            }
            if byte != expected {
                return false;
            }
        }
        offset == end - 1
    }
}
