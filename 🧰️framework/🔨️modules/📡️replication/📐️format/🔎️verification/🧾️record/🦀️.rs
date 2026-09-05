//! 🧾️ Scalar framing observations are not commit witnesses or input-reading authority.
//! The member history owner must pair them with its original verified retained input.

use super::{RetainedSprDiagnostic, RetainedSprVerification, Stage};

/// 📐️ Read-only current-frame coordinates; observing before its trailer grants no validity.
/// The raw-size scalar is not allocation credit; semantic consumers must enforce their own cap.
#[derive(Debug, PartialEq, Eq)]
pub struct RetainedSprRecordObservation {
    frame_start: u64,
    payload_start: u64,
    payload_end: u64,
    frame_end: u64,
    kind: u8,
    flags: u8,
    raw_bytes: Option<u64>,
}

impl RetainedSprRecordObservation {
    pub const fn frame_start(&self) -> u64 { self.frame_start }
    pub const fn payload_start(&self) -> u64 { self.payload_start }
    pub const fn payload_end(&self) -> u64 { self.payload_end }
    pub const fn frame_end(&self) -> u64 { self.frame_end }
    pub const fn kind(&self) -> u8 { self.kind }
    pub const fn flags(&self) -> u8 { self.flags }
    pub const fn raw_bytes(&self) -> Option<u64> { self.raw_bytes }
}

impl RetainedSprVerification {
    /// 👁️ Observe existing checked grammar state in constant space without parsing or advancing.
    pub fn observe_record_header(&self) -> Result<Option<RetainedSprRecordObservation>, RetainedSprDiagnostic> {
        if let Some(error) = self.error { return Err(error); }
        if !matches!(self.stage, Stage::Body | Stage::Trailer) || self.body_read < 2 || self.raw_len_pending { return Ok(None); }
        let body_start = self.frame_start.checked_add(u64::from(self.length_bytes)).ok_or(RetainedSprDiagnostic::Frame)?;
        let payload_start = body_start.checked_add(2 + u64::from(self.raw_len_bytes)).ok_or(RetainedSprDiagnostic::Frame)?;
        let payload_end = body_start.checked_add(self.body_len).ok_or(RetainedSprDiagnostic::Frame)?;
        let frame_end = payload_end.checked_add(8).ok_or(RetainedSprDiagnostic::Frame)?;
        if payload_start > payload_end || frame_end > self.total { return Err(RetainedSprDiagnostic::Frame); }
        Ok(Some(RetainedSprRecordObservation { frame_start: self.frame_start, payload_start, payload_end, frame_end,
            kind: self.kind, flags: self.flags, raw_bytes: (self.flags & 1 != 0).then_some(self.raw_len) }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::RetainedSprLimits;

    fn hex(value: &str) -> Vec<u8> { value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect() }

    #[test]
    fn retained_record_observation_uses_the_existing_framing_state_without_authority() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let mut body = vec![row["kind"].as_u64().unwrap() as u8, row["flags"].as_u64().unwrap() as u8];
            body.extend_from_slice(&hex(row["rawHex"].as_str().unwrap())); body.extend_from_slice(&hex(row["payloadHex"].as_str().unwrap()));
            body.resize(body.len() + row["repeat"].as_u64().unwrap() as usize, 97);
            let mut bytes = hex(fixture["headerHex"].as_str().unwrap()); crate::codec::write_varint_u64(&mut bytes, body.len() as u64);
            bytes.extend_from_slice(&body); bytes.extend_from_slice(&(crate::codec::crc32c(&body) ^ u32::from(row["corruptCrc"].as_bool().unwrap())).to_le_bytes());
            let frame_length = bytes.len() as u32 + 4 - 32; bytes.extend_from_slice(&frame_length.to_le_bytes());
            for grant in fixture["grants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
                let mut scanner = RetainedSprVerification::new(bytes.len() as u64, RetainedSprLimits::default()).unwrap();
                let mut error = None; let target = row["at"].as_u64().unwrap() as usize;
                while (scanner.consumed() as usize) < target {
                    let start = scanner.consumed() as usize; let mut fuel = grant;
                    if let Err(found) = scanner.push(&bytes[start..target], &mut fuel) { error = Some(found); break; }
                    assert_eq!(scanner.consumed() as usize - start, grant - fuel);
                }
                if row["cancel"].as_bool().unwrap() { scanner.cancel(); }
                let before = scanner.consumed(); let observed = scanner.observe_record_header();
                assert_eq!(scanner.observe_record_header(), observed); assert_eq!(scanner.consumed(), before);
                let found_error = observed.as_ref().err().copied().or(error).map(|error| match error { RetainedSprDiagnostic::Frame => "frame", RetainedSprDiagnostic::Cancelled => "cancelled", _ => panic!("unexpected observation diagnostic") });
                assert_eq!(found_error, row["error"].as_str(), "{}", row["id"]);
                let actual = observed.ok().flatten().map(|value| serde_json::json!({
                    "frameStart": value.frame_start(), "payloadStart": value.payload_start(), "payloadEnd": value.payload_end(),
                    "frameEnd": value.frame_end(), "kind": value.kind(), "flags": value.flags(), "rawBytes": value.raw_bytes()
                })).unwrap_or(serde_json::Value::Null);
                assert_eq!(actual, row["observation"], "{}", row["id"]);
                if found_error.is_none() {
                    let start = scanner.consumed() as usize; scanner.push(&bytes[start..], &mut bytes.len()).unwrap();
                    assert_eq!(scanner.observe_record_header().unwrap(), None);
                    let span = scanner.finish().unwrap(); assert_eq!(span.sequence(), 0); assert_eq!(span.end(), 32);
                    assert_eq!(scanner.observe_record_header().unwrap(), None);
                } else {
                    let mut fuel = grant; assert!(scanner.push(&[], &mut fuel).is_err()); assert_eq!(fuel, grant); assert_eq!(scanner.consumed(), before);
                }
            }
        }
        println!("[DEBUG] retained record observation: 11 neutral rows x3 grants; constant-state scalar metadata, unchanged position, empty/compressed/trailer/cancel/error, no commit or input authority");
    }
}
