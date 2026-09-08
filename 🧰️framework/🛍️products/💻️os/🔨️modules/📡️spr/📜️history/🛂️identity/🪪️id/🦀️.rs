//! 🪪️ Fixed-buffer tagged-id grammar with explicit bounded dictionary input, not dictionary authority.
//! The retained owner must bind each lookup to its verified dictionary and check StepContext.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HistoryIdDiagnostic {
    Identity,
    Malformed,
    Capacity,
    State,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Tag,
    Number,
    Raw,
    Lookup,
    Dictionary,
    Uuid,
    Done,
}

/// 🔤️ One wire/dictionary byte per fuel; UTF-8 and UUID expansion use only fixed storage.
pub(crate) struct RetainedHistoryIdV1 {
    output: [u8; 256],
    length: usize,
    stage: Stage,
    tag: u8,
    value: u64,
    digits: u8,
    remaining: usize,
    uuid_index: u8,
    utf_remaining: u8,
    utf_code: u32,
    utf_minimum: u32,
    error: Option<HistoryIdDiagnostic>,
}

impl RetainedHistoryIdV1 {
    pub(crate) fn new() -> Self {
        Self { output: [0; 256], length: 0, stage: Stage::Tag, tag: 0, value: 0, digits: 0, remaining: 0, uuid_index: 0, utf_remaining: 0, utf_code: 0, utf_minimum: 0, error: None }
    }

    fn reject<T>(&mut self, error: HistoryIdDiagnostic) -> Result<T, HistoryIdDiagnostic> {
        Err(*self.error.get_or_insert(error))
    }
    pub(crate) fn cancel(&mut self) {
        self.error.get_or_insert(HistoryIdDiagnostic::Cancelled);
    }
    pub(crate) fn lookup(&self) -> Option<u32> {
        (self.error.is_none() && self.stage == Stage::Lookup).then_some(self.value as u32)
    }
    pub(crate) fn dictionary_pending(&self) -> bool {
        self.error.is_none() && self.stage == Stage::Dictionary
    }
    pub(crate) fn is_complete(&self) -> bool {
        self.error.is_none() && self.stage == Stage::Done
    }

    pub(crate) fn push_wire(&mut self, byte: u8, fuel: &mut usize) -> Result<bool, HistoryIdDiagnostic> {
        if let Some(error) = self.error {
            return Err(error);
        }
        if *fuel == 0 {
            return Ok(false);
        }
        *fuel -= 1;
        let result = self.wire(byte);
        match result {
            Ok(()) => Ok(true),
            Err(error) => self.reject(error),
        }
    }

    fn wire(&mut self, byte: u8) -> Result<(), HistoryIdDiagnostic> {
        match self.stage {
            Stage::Tag => {
                if byte > 2 {
                    return Err(HistoryIdDiagnostic::Malformed);
                }
                self.tag = byte;
                self.stage = Stage::Number;
            }
            Stage::Number => {
                if self.digits == 9 && byte > 1 {
                    return Err(HistoryIdDiagnostic::Malformed);
                }
                self.value |= u64::from(byte & 127) << (u32::from(self.digits) * 7);
                self.digits += 1;
                if byte < 128 {
                    if self.digits > 1 && byte == 0 {
                        return Err(HistoryIdDiagnostic::Malformed);
                    }
                    if self.tag == 0 {
                        if self.value > 256 {
                            return Err(HistoryIdDiagnostic::Capacity);
                        }
                        if self.value == 0 {
                            return Err(HistoryIdDiagnostic::Identity);
                        }
                        self.remaining = self.value as usize;
                        self.stage = Stage::Raw;
                    } else {
                        if self.value >= 8192 {
                            return Err(HistoryIdDiagnostic::Capacity);
                        }
                        self.stage = Stage::Lookup;
                    }
                }
            }
            Stage::Raw => {
                self.text_byte(byte)?;
                self.remaining -= 1;
                if self.remaining == 0 {
                    self.end_text()?;
                    self.stage = Stage::Done;
                }
            }
            Stage::Uuid => {
                const HEX: &[u8; 16] = b"0123456789abcdef";
                self.text_byte(HEX[(byte >> 4) as usize])?;
                self.text_byte(HEX[(byte & 15) as usize])?;
                if matches!(self.uuid_index, 3 | 5 | 7 | 9) {
                    self.text_byte(b'-')?;
                }
                self.uuid_index += 1;
                if self.uuid_index == 16 {
                    self.stage = Stage::Done;
                }
            }
            Stage::Done => return Err(HistoryIdDiagnostic::Malformed),
            Stage::Lookup | Stage::Dictionary => return Err(HistoryIdDiagnostic::State),
        }
        Ok(())
    }

    pub(crate) fn begin_dictionary(&mut self, index: u32, bytes: usize, fuel: &mut usize) -> Result<bool, HistoryIdDiagnostic> {
        if let Some(error) = self.error {
            return Err(error);
        }
        if *fuel == 0 {
            return Ok(false);
        }
        *fuel -= 1;
        if self.stage != Stage::Lookup || u64::from(index) != self.value {
            return self.reject(HistoryIdDiagnostic::State);
        }
        if bytes == 0 {
            return self.reject(HistoryIdDiagnostic::Identity);
        }
        if bytes > if self.tag == 2 { 219 } else { 256 } {
            return self.reject(HistoryIdDiagnostic::Capacity);
        }
        self.remaining = bytes;
        self.stage = Stage::Dictionary;
        Ok(true)
    }

    pub(crate) fn push_dictionary(&mut self, byte: u8, fuel: &mut usize) -> Result<bool, HistoryIdDiagnostic> {
        if let Some(error) = self.error {
            return Err(error);
        }
        if *fuel == 0 {
            return Ok(false);
        }
        *fuel -= 1;
        if self.stage != Stage::Dictionary {
            return self.reject(HistoryIdDiagnostic::State);
        }
        if let Err(error) = self.text_byte(byte) {
            return self.reject(error);
        }
        self.remaining -= 1;
        if self.remaining == 0 {
            if let Err(error) = self.end_text() {
                return self.reject(error);
            }
            if self.tag == 2 {
                if let Err(error) = self.text_byte(b'-') {
                    return self.reject(error);
                }
                self.stage = Stage::Uuid;
            } else {
                self.stage = Stage::Done;
            }
        }
        Ok(true)
    }

    fn text_byte(&mut self, byte: u8) -> Result<(), HistoryIdDiagnostic> {
        if self.length == self.output.len() {
            return Err(HistoryIdDiagnostic::Capacity);
        }
        self.output[self.length] = byte;
        self.length += 1;
        if self.utf_remaining == 0 {
            match byte {
                0..=127 => {
                    self.utf_code = u32::from(byte);
                    self.utf_minimum = 0;
                }
                194..=223 => {
                    self.utf_code = u32::from(byte & 31);
                    self.utf_minimum = 128;
                    self.utf_remaining = 1;
                }
                224..=239 => {
                    self.utf_code = u32::from(byte & 15);
                    self.utf_minimum = 2048;
                    self.utf_remaining = 2;
                }
                240..=244 => {
                    self.utf_code = u32::from(byte & 7);
                    self.utf_minimum = 65536;
                    self.utf_remaining = 3;
                }
                _ => return Err(HistoryIdDiagnostic::Malformed),
            }
        } else {
            if byte & 192 != 128 {
                return Err(HistoryIdDiagnostic::Malformed);
            }
            self.utf_code = (self.utf_code << 6) | u32::from(byte & 63);
            self.utf_remaining -= 1;
        }
        if self.utf_remaining == 0 {
            if self.utf_code < self.utf_minimum {
                return Err(HistoryIdDiagnostic::Malformed);
            }
            let scalar = char::from_u32(self.utf_code).ok_or(HistoryIdDiagnostic::Malformed)?;
            if scalar.is_control() {
                return Err(HistoryIdDiagnostic::Identity);
            }
        }
        Ok(())
    }

    fn end_text(&self) -> Result<(), HistoryIdDiagnostic> {
        if self.utf_remaining != 0 {
            return Err(HistoryIdDiagnostic::Malformed);
        }
        Ok(())
    }

    pub(crate) fn finish(&mut self) -> Result<&str, HistoryIdDiagnostic> {
        if let Some(error) = self.error {
            return Err(error);
        }
        if self.stage != Stage::Done {
            return self.reject(HistoryIdDiagnostic::Malformed);
        }
        std::str::from_utf8(&self.output[..self.length]).map_err(|_| HistoryIdDiagnostic::Malformed)
    }

    pub(crate) fn close_bytes(&mut self, maximum: usize) -> usize {
        self.cancel();
        let released = maximum.min(self.length);
        self.output[self.length - released..self.length].fill(0);
        self.length -= released;
        if self.length == 0 {
            self.utf_code = 0;
            self.utf_minimum = 0;
            self.value = 0;
        }
        released
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
