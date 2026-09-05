//! 🧾️ Bounded dictionary payload grammar; emitted ranges are scalar facts, never read authority.
//! The input owner must adopt each event before feeding another byte and publish only after EOF.

#[derive(Debug, PartialEq, Eq)]
pub(super) enum DictionaryDeltaEvent {
    Begin { base: u64, count: u64 },
    Entry { offset: u64, length: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DictionaryDeltaError {
    Malformed,
    Capacity,
    State,
    Cancelled,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Format,
    Base,
    Count,
    Length,
    Text,
    Done,
}

/// 🔡️ Constant-space UTF-8/LEB128 cursor with one payload byte per fuel and retained events.
pub(super) struct RetainedDictionaryDelta {
    position: u64,
    end: u64,
    stage: Stage,
    number: u64,
    digits: u8,
    base: u64,
    entries: u64,
    remaining: u64,
    entry_start: u64,
    entry_length: u64,
    utf8: [u8; 4],
    utf8_used: usize,
    utf8_expected: usize,
    event: Option<DictionaryDeltaEvent>,
    diagnostic: Option<DictionaryDeltaError>,
    closed: bool,
}

impl RetainedDictionaryDelta {
    pub(super) fn new(start: u64, end: u64) -> Result<Self, DictionaryDeltaError> {
        if start >= end {
            return Err(DictionaryDeltaError::Malformed);
        }
        Ok(Self { position: start, end, stage: Stage::Format, number: 0, digits: 0, base: 0, entries: 0, remaining: 0, entry_start: 0, entry_length: 0, utf8: [0; 4], utf8_used: 0, utf8_expected: 0, event: None, diagnostic: None, closed: false })
    }

    fn reject<T>(&mut self, diagnostic: DictionaryDeltaError) -> Result<T, DictionaryDeltaError> {
        Err(*self.diagnostic.get_or_insert(diagnostic))
    }
    fn check(&self) -> Result<(), DictionaryDeltaError> {
        if self.closed {
            return Err(DictionaryDeltaError::State);
        }
        self.diagnostic.map_or(Ok(()), Err)
    }
    pub(super) fn next_offset(&self) -> u64 {
        self.position
    }
    pub(super) fn has_event(&self) -> bool {
        self.event.is_some()
    }
    pub(super) fn take_event(&mut self) -> Result<Option<DictionaryDeltaEvent>, DictionaryDeltaError> {
        self.check()?;
        Ok(self.event.take())
    }
    pub(super) fn cancel(&mut self) {
        self.diagnostic.get_or_insert(DictionaryDeltaError::Cancelled);
    }

    pub(super) fn push(&mut self, byte: u8, fuel: &mut usize) -> Result<bool, DictionaryDeltaError> {
        self.check()?;
        if *fuel == 0 {
            return Ok(false);
        }
        if self.event.is_some() {
            return self.reject(DictionaryDeltaError::State);
        }
        if self.position >= self.end || self.stage == Stage::Done {
            return self.reject(DictionaryDeltaError::Malformed);
        }
        *fuel -= 1;
        self.position += 1;
        if let Err(diagnostic) = self.byte(byte) {
            return self.reject(diagnostic);
        }
        Ok(true)
    }

    fn byte(&mut self, byte: u8) -> Result<(), DictionaryDeltaError> {
        match self.stage {
            Stage::Format => {
                if byte != 1 {
                    return Err(DictionaryDeltaError::Malformed);
                }
                self.stage = Stage::Base;
            }
            Stage::Base | Stage::Count | Stage::Length => {
                if self.digits == 9 && byte > 1 {
                    return Err(DictionaryDeltaError::Malformed);
                }
                self.number |= u64::from(byte & 127) << (u32::from(self.digits) * 7);
                self.digits += 1;
                if byte < 128 {
                    if self.digits > 1 && byte == 0 {
                        return Err(DictionaryDeltaError::Malformed);
                    }
                    let value = self.number;
                    self.number = 0;
                    self.digits = 0;
                    match self.stage {
                        Stage::Base => {
                            self.base = value;
                            self.stage = Stage::Count;
                        }
                        Stage::Count => {
                            if value > 8192 {
                                return Err(DictionaryDeltaError::Capacity);
                            }
                            self.entries = value;
                            self.stage = if value == 0 { Stage::Done } else { Stage::Length };
                            self.event = Some(DictionaryDeltaEvent::Begin { base: self.base, count: value });
                        }
                        Stage::Length => {
                            if value > 1_048_576 {
                                return Err(DictionaryDeltaError::Capacity);
                            }
                            if value > self.end - self.position {
                                return Err(DictionaryDeltaError::Malformed);
                            }
                            self.entry_start = self.position;
                            self.entry_length = value;
                            self.remaining = value;
                            if value == 0 {
                                self.complete_entry();
                            } else {
                                self.stage = Stage::Text;
                            }
                        }
                        _ => unreachable!("number stage was matched"),
                    }
                }
            }
            Stage::Text => {
                self.text_byte(byte)?;
                self.remaining -= 1;
                if self.remaining == 0 {
                    if self.utf8_used != 0 {
                        return Err(DictionaryDeltaError::Malformed);
                    }
                    self.complete_entry();
                }
            }
            Stage::Done => return Err(DictionaryDeltaError::Malformed),
        }
        Ok(())
    }

    fn text_byte(&mut self, byte: u8) -> Result<(), DictionaryDeltaError> {
        if self.utf8_used == 0 {
            if byte < 128 {
                return Ok(());
            }
            self.utf8_expected = match byte {
                194..=223 => 2,
                224..=239 => 3,
                240..=244 => 4,
                _ => return Err(DictionaryDeltaError::Malformed),
            };
        } else if byte & 192 != 128 {
            return Err(DictionaryDeltaError::Malformed);
        }
        self.utf8[self.utf8_used] = byte;
        self.utf8_used += 1;
        if self.utf8_used == self.utf8_expected {
            std::str::from_utf8(&self.utf8[..self.utf8_used]).map_err(|_| DictionaryDeltaError::Malformed)?;
            self.utf8.fill(0);
            self.utf8_used = 0;
            self.utf8_expected = 0;
        }
        Ok(())
    }

    fn complete_entry(&mut self) {
        self.entries -= 1;
        self.event = Some(DictionaryDeltaEvent::Entry { offset: self.entry_start, length: self.entry_length });
        self.stage = if self.entries == 0 { Stage::Done } else { Stage::Length };
    }

    pub(super) fn finish(&mut self) -> Result<(), DictionaryDeltaError> {
        self.check()?;
        if self.event.is_some() {
            return self.reject(DictionaryDeltaError::State);
        }
        if self.position != self.end || self.stage != Stage::Done {
            return self.reject(DictionaryDeltaError::Malformed);
        }
        Ok(())
    }

    pub(super) fn close_bytes(&mut self, maximum: usize) -> usize {
        self.closed = true;
        self.event = None;
        let count = maximum.min(self.utf8_used);
        self.utf8[self.utf8_used - count..self.utf8_used].fill(0);
        self.utf8_used -= count;
        if self.utf8_used == 0 {
            self.utf8_expected = 0;
            self.number = 0;
        }
        count
    }
    pub(super) fn retained_scratch_bytes(&self) -> usize {
        self.utf8_used
    }
    pub(super) fn terminal_is_empty(&self) -> bool {
        self.closed && self.utf8_used == 0 && self.event.is_none()
    }
}

impl Drop for RetainedDictionaryDelta {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "dictionary payload scratch requires bounded retirement");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(cursor: &mut RetainedDictionaryDelta, grant: usize) -> usize {
        let expected = cursor.retained_scratch_bytes();
        let mut retired = cursor.close_bytes(0);
        assert_eq!(retired, 0);
        while !cursor.terminal_is_empty() {
            let count = cursor.close_bytes(grant);
            assert!(count <= grant);
            retired += count;
        }
        assert_eq!(retired, expected);
        assert!(cursor.utf8.iter().all(|byte| *byte == 0));
        retired
    }

    fn verify_payload_fixture(accepted: bool) -> usize {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).unwrap();
        let mut selected = 0;
        for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["error"].is_null() == accepted) {
            selected += 1;
            let name = row["id"].as_str().unwrap();
            let hex = row["hex"].as_str().unwrap();
            let mut bytes: Vec<u8> = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap()).collect();
            if let Some(repeat) = row.get("repeat") {
                bytes.resize(bytes.len() + repeat["count"].as_u64().unwrap() as usize, repeat["byte"].as_u64().unwrap() as u8);
            }
            for grant in fixture["grants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
                let mut cursor = RetainedDictionaryDelta::new(0, bytes.len() as u64).unwrap();
                let mut zero = 0;
                assert!(!cursor.push(bytes[0], &mut zero).unwrap());
                assert_eq!(cursor.next_offset(), 0);
                let mut error = None;
                let mut entries = 0;
                let mut events = Vec::new();
                'feed: while (cursor.next_offset() as usize) < bytes.len() {
                    let mut fuel = grant;
                    while fuel > 0 && (cursor.next_offset() as usize) < bytes.len() {
                        let before = cursor.next_offset();
                        let before_fuel = fuel;
                        let pending = cursor.has_event();
                        if let Err(found) = cursor.push(bytes[before as usize], &mut fuel) {
                            if pending {
                                assert_eq!(cursor.next_offset(), before, "{name}");
                                assert_eq!(fuel, before_fuel, "{name}");
                            }
                            error = Some(found);
                            break 'feed;
                        }
                        assert_eq!(cursor.next_offset(), before + 1, "{name}");
                        assert_eq!(fuel + 1, before_fuel, "{name}");
                        let event = match cursor.event.as_ref() {
                            Some(DictionaryDeltaEvent::Begin { base, count }) => {
                                events.push(serde_json::json!(["begin", base, count]));
                                Some("begin")
                            }
                            Some(DictionaryDeltaEvent::Entry { offset, length }) => {
                                entries += 1;
                                events.push(serde_json::json!(["entry", offset, length]));
                                Some("entry")
                            }
                            None => None,
                        };
                        if row["cancelAt"].as_u64() == Some(cursor.next_offset()) {
                            cursor.cancel();
                            error = cursor.finish().err();
                            break 'feed;
                        }
                        if event != row["hold"].as_str() {
                            cursor.take_event().unwrap();
                            assert!(!cursor.has_event());
                        }
                    }
                }
                let error = error.or_else(|| cursor.finish().err());
                if let Some(expected) = error {
                    let before = cursor.next_offset();
                    let mut fuel = grant;
                    assert_eq!(cursor.push(0, &mut fuel), Err(expected), "{name}");
                    assert_eq!(cursor.next_offset(), before);
                    assert_eq!(fuel, grant);
                }
                let position = cursor.next_offset();
                let scratch = cursor.retained_scratch_bytes();
                let retired = close(&mut cursor, grant);
                let actual = error.map(|error| match error {
                    DictionaryDeltaError::Malformed => "malformed",
                    DictionaryDeltaError::Capacity => "capacity",
                    DictionaryDeltaError::State => "state",
                    DictionaryDeltaError::Cancelled => "cancelled",
                });
                let mut expected_events = Vec::new();
                for event in fixture["events"][name].as_array().unwrap() {
                    if event[0] == "begin" {
                        expected_events.push(event.clone());
                    } else {
                        for index in 0..event[3].as_u64().unwrap() {
                            expected_events.push(serde_json::json!(["entry", event[1].as_u64().unwrap() + index * event[4].as_u64().unwrap(), event[2]]));
                        }
                    }
                }
                assert_eq!(events, expected_events, "{name}");
                assert_eq!(actual, row["error"].as_str(), "{name}");
                assert_eq!(position, row["offset"].as_u64().unwrap(), "{name}");
                assert_eq!(entries, row["entries"].as_u64().unwrap(), "{name}");
                assert_eq!(scratch as u64, row["scratchBytes"].as_u64().unwrap(), "{name}");
                assert_eq!(retired as u64, row["scratchBytes"].as_u64().unwrap(), "{name}");
            }
        }
        selected
    }

    #[test]
    fn retained_dictionary_delta_matches_neutral_text_ranges_without_publication() {
        assert_eq!(verify_payload_fixture(true), 11);
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap();
        for (base, name) in [(0, "dictionary"), (7, "secondDictionary")] {
            let entries = fixture[name].as_array().unwrap();
            let mut bytes = vec![1, base, entries.len() as u8];
            for entry in entries {
                let value = entry.as_str().unwrap().as_bytes();
                bytes.push(value.len() as u8);
                bytes.extend_from_slice(value);
            }
            for grant in [1, 7, 4096] {
                let mut cursor = RetainedDictionaryDelta::new(32, 32 + bytes.len() as u64).unwrap();
                let mut ranges = Vec::new();
                let mut offset = 0;
                let mut zero = 0;
                assert!(!cursor.push(bytes[0], &mut zero).unwrap());
                assert_eq!(cursor.next_offset(), 32);
                while offset < bytes.len() {
                    let mut fuel = grant;
                    while fuel > 0 && offset < bytes.len() {
                        assert!(cursor.push(bytes[offset], &mut fuel).unwrap());
                        offset += 1;
                        match cursor.take_event().unwrap() {
                            Some(DictionaryDeltaEvent::Begin { base: actual, count }) => {
                                assert_eq!(actual, u64::from(base));
                                assert_eq!(count, entries.len() as u64);
                            }
                            Some(DictionaryDeltaEvent::Entry { offset, length }) => ranges.push((offset, length)),
                            None => {}
                        }
                    }
                }
                cursor.finish().unwrap();
                assert_eq!(ranges.len(), entries.len());
                for ((offset, length), entry) in ranges.into_iter().zip(entries) {
                    assert_eq!(&bytes[(offset - 32) as usize..(offset + length - 32) as usize], entry.as_str().unwrap().as_bytes());
                }
                assert_eq!(close(&mut cursor, grant), 0);
            }
        }
        println!("[DEBUG] dictionary payload cursor: 11 neutral accepted wires x3 grants; exact ranges, UTF8, canonical LEB, no publication");
    }

    #[test]
    fn retained_dictionary_delta_rejects_tail_and_preserves_partial_utf8_until_close() {
        assert_eq!(verify_payload_fixture(false), 23);
        for bytes in [&[1, 0, 1, 2, 195, 40][..], &[1, 0, 1, 1, 195], &[1, 0, 1, 128, 0], &[1, 0, 0, 0]] {
            let mut cursor = RetainedDictionaryDelta::new(0, bytes.len() as u64).unwrap();
            let mut error = None;
            for byte in bytes {
                let mut fuel = 1;
                if let Err(found) = cursor.push(*byte, &mut fuel) {
                    error = Some(found);
                    break;
                }
                cursor.take_event().unwrap();
            }
            assert_eq!(error.or_else(|| cursor.finish().err()), Some(DictionaryDeltaError::Malformed));
            close(&mut cursor, 1);
        }
        let bytes = [1, 0, 1, 4, 240, 159, 152, 128];
        for boundary in 0..=bytes.len() {
            let mut cursor = RetainedDictionaryDelta::new(0, bytes.len() as u64).unwrap();
            for byte in &bytes[..boundary] {
                let mut fuel = 1;
                cursor.push(*byte, &mut fuel).unwrap();
                cursor.take_event().unwrap();
            }
            cursor.cancel();
            assert_eq!(cursor.finish(), Err(DictionaryDeltaError::Cancelled));
            close(&mut cursor, 1);
        }
        for bytes in [&[1, 0, 1, 0][..], &[1, 0, 1, 2, 194, 133]] {
            let mut cursor = RetainedDictionaryDelta::new(0, bytes.len() as u64).unwrap();
            for byte in bytes {
                let mut fuel = 1;
                cursor.push(*byte, &mut fuel).unwrap();
                cursor.take_event().unwrap();
            }
            cursor.finish().unwrap();
            close(&mut cursor, 1);
        }
        println!("[DEBUG] dictionary payload cursor: 23 neutral denied wires x3 grants; sticky errors/event fences and exact scratch0..4 retirement; every four-byte scalar cancellation boundary");
    }
}
