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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
