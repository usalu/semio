//! 🔐️ String and hex authorities: the bounded string authority and the hexadecimal authority with their steps. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaStringStep {
    Pending,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

#[derive(Clone, Copy)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaStringState {
    Raw,
    Escape,
    Unicode { value: u16, digits: u8 },
    SurrogateSlash { high: u16 },
    SurrogateU { high: u16 },
    SurrogateLow { high: u16, value: u16, digits: u8 },
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

pub struct OwnedSchemaStringAuthority<const MAXIMUM_BYTES: usize> {
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) token: OwnedSchemaToken,
    pub(super) path: OwnedSchemaPath,
    relative: usize,
    pub(super) state: OwnedSchemaStringState,
    pub(super) bytes: [u8; MAXIMUM_BYTES],
    pub(super) len: usize,
    taken: bool,
}

impl<const MAXIMUM_BYTES: usize> OwnedSchemaStringAuthority<MAXIMUM_BYTES> {
    /// 🔠️ Binds one string token to fixed caller-chosen semantic storage.
    pub fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, token: OwnedSchemaToken, path: OwnedSchemaPath) -> Result<Self, OwnedSchemaToken> {
        if token.kind != OwnedSchemaTokenKind::String || token.end.saturating_sub(token.start) < 2 || MAXIMUM_BYTES == 0 {
            return Err(token);
        }
        Ok(Self { operation, generation, token, path, relative: 1, state: OwnedSchemaStringState::Raw, bytes: [0; MAXIMUM_BYTES], len: 0, taken: false })
    }

    pub(super) fn diagnostic(&self, code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: self.token.start + self.relative as u64, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn raw_byte(&self, source: &OwnedSchemaRecordCursor) -> Option<u8> {
        let mut byte = [0];
        (source.copy_token_bytes(self.token, self.relative, &mut byte) == 1).then_some(byte[0])
    }

    fn push_byte(&mut self, byte: u8) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.len == MAXIMUM_BYTES {
            return Err(self.diagnostic("schema-json.string-byte-capacity"));
        }
        self.bytes[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    fn push_scalar(&mut self, scalar: u32) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        let Some(value) = char::from_u32(scalar) else {
            return Err(self.diagnostic("schema-json.invalid-unicode-scalar"));
        };
        let mut encoded = [0; 4];
        for byte in value.encode_utf8(&mut encoded).bytes() {
            self.push_byte(byte)?;
        }
        Ok(())
    }

    fn hex(byte: u8) -> Option<u16> {
        match byte {
            b'0'..=b'9' => Some(u16::from(byte - b'0')),
            b'a'..=b'f' => Some(u16::from(byte - b'a' + 10)),
            b'A'..=b'F' => Some(u16::from(byte - b'A' + 10)),
            _ => None,
        }
    }

    fn finish_unicode(&mut self, value: u16) -> Result<OwnedSchemaStringState, OwnedSchemaDecodeDiagnostic> {
        match value {
            0xd800..=0xdbff => Ok(OwnedSchemaStringState::SurrogateSlash { high: value }),
            0xdc00..=0xdfff => Err(self.diagnostic("schema-json.unpaired-low-surrogate")),
            _ => {
                self.push_scalar(u32::from(value))?;
                Ok(OwnedSchemaStringState::Raw)
            }
        }
    }

    /// ⏭️ Unescapes source bytes one semantic byte/scalar at a time under the caller's grant.
    pub fn step(&mut self, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaStringStep {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            let diagnostic = self.diagnostic("schema-json.stale-string-authority");
            self.state = OwnedSchemaStringState::Fault(diagnostic);
            return OwnedSchemaStringStep::Fault(diagnostic);
        }
        if cx.is_cancelled() {
            self.state = OwnedSchemaStringState::Cancelled;
            return OwnedSchemaStringStep::Cancelled;
        }
        cx.set_stage("schema-json-string");
        loop {
            if cx.should_yield() {
                return OwnedSchemaStringStep::Pending;
            }
            match self.state {
                OwnedSchemaStringState::Complete => return OwnedSchemaStringStep::Complete,
                OwnedSchemaStringState::Cancelled => return OwnedSchemaStringStep::Cancelled,
                OwnedSchemaStringState::Fault(diagnostic) => return OwnedSchemaStringStep::Fault(diagnostic),
                _ => {}
            }
            if self.relative + 1 == self.token.end.saturating_sub(self.token.start) as usize {
                if matches!(self.state, OwnedSchemaStringState::Raw) {
                    self.state = OwnedSchemaStringState::Complete;
                    return OwnedSchemaStringStep::Complete;
                }
                let diagnostic = self.diagnostic("schema-json.truncated-string-escape");
                self.state = OwnedSchemaStringState::Fault(diagnostic);
                return OwnedSchemaStringStep::Fault(diagnostic);
            }
            let Some(byte) = self.raw_byte(source) else {
                let diagnostic = self.diagnostic("schema-json.string-source-offset");
                self.state = OwnedSchemaStringState::Fault(diagnostic);
                return OwnedSchemaStringStep::Fault(diagnostic);
            };
            let next = match self.state {
                OwnedSchemaStringState::Raw if byte == b'\\' => Ok(OwnedSchemaStringState::Escape),
                OwnedSchemaStringState::Raw => self.push_byte(byte).map(|_| OwnedSchemaStringState::Raw),
                OwnedSchemaStringState::Escape => match byte {
                    b'"' | b'\\' | b'/' => self.push_byte(byte).map(|_| OwnedSchemaStringState::Raw),
                    b'b' => self.push_byte(0x08).map(|_| OwnedSchemaStringState::Raw),
                    b'f' => self.push_byte(0x0c).map(|_| OwnedSchemaStringState::Raw),
                    b'n' => self.push_byte(b'\n').map(|_| OwnedSchemaStringState::Raw),
                    b'r' => self.push_byte(b'\r').map(|_| OwnedSchemaStringState::Raw),
                    b't' => self.push_byte(b'\t').map(|_| OwnedSchemaStringState::Raw),
                    b'u' => Ok(OwnedSchemaStringState::Unicode { value: 0, digits: 0 }),
                    _ => Err(self.diagnostic("schema-json.invalid-string-escape")),
                },
                OwnedSchemaStringState::Unicode { value, digits } => match Self::hex(byte) {
                    Some(nibble) if digits == 3 => self.finish_unicode((value << 4) | nibble),
                    Some(nibble) => Ok(OwnedSchemaStringState::Unicode { value: (value << 4) | nibble, digits: digits + 1 }),
                    None => Err(self.diagnostic("schema-json.invalid-string-unicode")),
                },
                OwnedSchemaStringState::SurrogateSlash { high } if byte == b'\\' => Ok(OwnedSchemaStringState::SurrogateU { high }),
                OwnedSchemaStringState::SurrogateSlash { .. } => Err(self.diagnostic("schema-json.missing-low-surrogate")),
                OwnedSchemaStringState::SurrogateU { high } if byte == b'u' => Ok(OwnedSchemaStringState::SurrogateLow { high, value: 0, digits: 0 }),
                OwnedSchemaStringState::SurrogateU { .. } => Err(self.diagnostic("schema-json.missing-low-surrogate")),
                OwnedSchemaStringState::SurrogateLow { high, value, digits } => match Self::hex(byte) {
                    Some(nibble) if digits == 3 => {
                        let low = (value << 4) | nibble;
                        if !(0xdc00..=0xdfff).contains(&low) {
                            Err(self.diagnostic("schema-json.invalid-low-surrogate"))
                        } else {
                            let scalar = 0x1_0000 + ((u32::from(high) - 0xd800) << 10) + (u32::from(low) - 0xdc00);
                            self.push_scalar(scalar).map(|_| OwnedSchemaStringState::Raw)
                        }
                    }
                    Some(nibble) => Ok(OwnedSchemaStringState::SurrogateLow { high, value: (value << 4) | nibble, digits: digits + 1 }),
                    None => Err(self.diagnostic("schema-json.invalid-low-surrogate")),
                },
                OwnedSchemaStringState::Complete | OwnedSchemaStringState::Cancelled | OwnedSchemaStringState::Fault(_) => unreachable!("terminal string state returned before source read"),
            };
            self.relative += 1;
            cx.consume_fuel(1);
            match next {
                Ok(state) => self.state = state,
                Err(diagnostic) => {
                    self.state = OwnedSchemaStringState::Fault(diagnostic);
                    return OwnedSchemaStringStep::Fault(diagnostic);
                }
            }
            if cx.is_cancelled() {
                self.state = OwnedSchemaStringState::Cancelled;
                return OwnedSchemaStringStep::Cancelled;
            }
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        matches!(self.state, OwnedSchemaStringState::Complete).then(|| unsafe { std::str::from_utf8_unchecked(&self.bytes[..self.len]) })
    }

    /// 📤️ Publishes the bounded semantic string exactly once after terminal validation.
    pub fn take_string(&mut self) -> Option<String> {
        if self.taken || !matches!(self.state, OwnedSchemaStringState::Complete) {
            return None;
        }
        self.taken = true;
        Some(unsafe { String::from_utf8_unchecked(self.bytes[..self.len].to_vec()) })
    }

    /// 🛑️ Releases the definitionally inline partial scalar without allocating.
    pub fn cancel(&mut self) {
        self.state = OwnedSchemaStringState::Cancelled;
        self.taken = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken || matches!(self.state, OwnedSchemaStringState::Cancelled | OwnedSchemaStringState::Fault(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaHexStep {
    Pending,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

/// 🔢 Incremental bounded hex scalar used by owner catalogs for nested pack values.
/// The decoded bytes stay inline and no String/Vec is ever constructed.
pub struct OwnedSchemaHexAuthority<const MAXIMUM_BYTES: usize> {
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) token: OwnedSchemaToken,
    pub(super) path: OwnedSchemaPath,
    relative: usize,
    pub(super) bytes: Box<[u8; MAXIMUM_BYTES]>,
    pub(super) len: usize,
    high: Option<u8>,
    complete: bool,
    taken: bool,
}

impl<const MAXIMUM_BYTES: usize> OwnedSchemaHexAuthority<MAXIMUM_BYTES> {
    pub fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, token: OwnedSchemaToken, path: OwnedSchemaPath) -> Result<Self, OwnedSchemaDecodeDiagnostic> {
        if token.kind != OwnedSchemaTokenKind::String {
            return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.expected-hex-string", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        Ok(Self { operation, generation, token, path, relative: 1, bytes: Box::new([0; MAXIMUM_BYTES]), len: 0, high: None, complete: false, taken: false })
    }

    pub(super) fn fault(&self, code: &'static str) -> OwnedSchemaHexStep {
        OwnedSchemaHexStep::Fault(OwnedSchemaDecodeDiagnostic { code, offset: self.token.start + self.relative as u64, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })
    }

    fn nibble(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }

    pub fn step(&mut self, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaHexStep {
        if self.complete {
            return OwnedSchemaHexStep::Complete;
        }
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return self.fault("schema-json.stale-hex-authority");
        }
        if cx.is_cancelled() {
            self.taken = true;
            return OwnedSchemaHexStep::Cancelled;
        }
        cx.set_stage("schema-json-hex");
        while cx.fuel_remaining() != 0 {
            if self.token.start + self.relative as u64 + 1 >= self.token.end {
                if self.high.is_some() {
                    return self.fault("schema-json.odd-hex-length");
                }
                self.complete = true;
                return OwnedSchemaHexStep::Complete;
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(self.token, self.relative, &mut byte) != 1 {
                return self.fault("schema-json.truncated-hex");
            }
            let Some(nibble) = Self::nibble(byte[0]) else { return self.fault("schema-json.invalid-hex") };
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                if self.len == MAXIMUM_BYTES {
                    return self.fault("schema-json.hex-byte-capacity");
                }
                self.bytes[self.len] = (high << 4) | nibble;
                self.len += 1;
            } else {
                self.high = Some(nibble);
            }
        }
        OwnedSchemaHexStep::Pending
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        (self.complete && !self.taken).then_some(&self.bytes[..self.len])
    }

    pub fn release(&mut self) -> bool {
        if !self.complete || self.taken {
            return false;
        }
        self.taken = true;
        true
    }

    pub fn cancel(&mut self) {
        self.taken = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken
    }
}
