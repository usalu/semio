//! 🔎️ Caller-retained full SPR framing and commit verification before semantic hydration.
//! A torn or uncommitted suffix recovers the last verified commit; a complete invalid frame
//! or commit rejects. No record, dictionary, input owner or typed document is published here.
//! The retained profile is canonical v1.0, hash-chain-only, unsigned and unencrypted. It is
//! intentionally stricter than the generic FrameCursor and never falls back to that reader.
//! Compressed frames are structurally checked, not decompressed or granted raw allocation credit.
//! Reserved frame bits and inconsistent compression/codec flags are denied.

#[path = "🧾️record/🦀️.rs"]
pub mod record;

use crate::codec::Crc32cCursor;
use semio_framework_hash::Hasher;

/// 📏️ Allocation-independent ceilings applied before frame traversal.
#[derive(Clone, Copy, Debug)]
pub struct RetainedSprLimits {
    pub file_bytes: u64,
    pub frame_body_bytes: u64,
    pub records: u64,
}

impl Default for RetainedSprLimits {
    fn default() -> Self {
        Self { file_bytes: 64 * 1024 * 1024, frame_body_bytes: 1024 * 1024, records: 8192 }
    }
}

/// 🚦️ Closed diagnostics contain neither payload bytes nor artifact identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedSprDiagnostic {
    Header,
    Frame,
    Commit,
    Capacity,
    Cancelled,
    State,
}

/// 🧾️ Exact EOF-verified metadata with a private constructor, not an input-authority receipt.
#[derive(Debug, PartialEq, Eq)]
pub struct VerifiedSprSpan {
    end: u64,
    sequence: u64,
    commit_offset: u64,
    frames: u64,
    tail: u64,
    chain: [u8; 32],
}

impl VerifiedSprSpan {
    pub const fn end(&self) -> u64 { self.end }
    pub const fn sequence(&self) -> u64 { self.sequence }
    pub const fn commit_offset(&self) -> u64 { self.commit_offset }
    pub const fn frames(&self) -> u64 { self.frames }
    pub const fn tail(&self) -> u64 { self.tail }
    pub const fn chain(&self) -> &[u8; 32] { &self.chain }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage { Header, Length, Body, Trailer, Torn, Done }

/// 🧵️ Fixed-buffer one-byte fuel machine; callers retain input and check their own authority.
pub struct RetainedSprVerification {
    limits: RetainedSprLimits,
    total: u64,
    consumed: u64,
    stage: Stage,
    error: Option<RetainedSprDiagnostic>,
    header: [u8; 32],
    trailer: [u8; 8],
    commit: [u8; 64],
    frame_start: u64,
    body_len: u64,
    body_read: u64,
    length_bytes: u8,
    trailer_read: usize,
    raw_len: u64,
    raw_len_bytes: u8,
    raw_len_pending: bool,
    kind: u8,
    flags: u8,
    crc: Crc32cCursor,
    frame_hash: Hasher,
    pending_hash: Hasher,
    pending_bytes: u64,
    pending_records: u32,
    frames: u64,
    verified_frames: u64,
    verified_end: u64,
    sequence: u64,
    commit_offset: u64,
    chain: [u8; 32],
}

impl RetainedSprVerification {
    pub fn new(total: u64, limits: RetainedSprLimits) -> Result<Self, RetainedSprDiagnostic> {
        if total > limits.file_bytes || limits.frame_body_bytes < 2 || limits.records == 0 {
            return Err(RetainedSprDiagnostic::Capacity);
        }
        Ok(Self {
            limits, total, consumed: 0, stage: Stage::Header, error: None, header: [0; 32], trailer: [0; 8], commit: [0; 64],
            frame_start: 32, body_len: 0, body_read: 0, length_bytes: 0, trailer_read: 0,
            raw_len: 0, raw_len_bytes: 0, raw_len_pending: false, kind: 0, flags: 0,
            crc: Crc32cCursor::new(), frame_hash: Hasher::new(), pending_hash: Hasher::new(),
            pending_bytes: 0, pending_records: 0, frames: 0, verified_frames: 0, verified_end: 32,
            sequence: 0, commit_offset: 0, chain: [0; 32],
        })
    }

    pub const fn consumed(&self) -> u64 { self.consumed }

    pub fn cancel(&mut self) {
        if self.error.is_none() { self.error = Some(RetainedSprDiagnostic::Cancelled); }
    }

    pub fn push(&mut self, input: &[u8], fuel: &mut usize) -> Result<usize, RetainedSprDiagnostic> {
        if let Some(error) = self.error { return Err(error); }
        if self.stage == Stage::Done { return Err(RetainedSprDiagnostic::State); }
        if input.len() as u64 > self.total - self.consumed { return self.reject(RetainedSprDiagnostic::State); }
        let count = input.len().min(*fuel);
        for &byte in &input[..count] {
            *fuel -= 1;
            self.consumed += 1;
            if let Err(error) = self.byte(byte) { return self.reject(error); }
        }
        Ok(count)
    }

    pub fn finish(&mut self) -> Result<VerifiedSprSpan, RetainedSprDiagnostic> {
        if let Some(error) = self.error { return Err(error); }
        if self.stage == Stage::Done || self.consumed != self.total { return Err(RetainedSprDiagnostic::State); }
        if self.stage == Stage::Header { return self.reject(RetainedSprDiagnostic::Header); }
        self.stage = Stage::Done;
        Ok(VerifiedSprSpan { end: self.verified_end, sequence: self.sequence, commit_offset: self.commit_offset,
            frames: self.verified_frames, tail: self.total - self.verified_end, chain: self.chain })
    }

    fn reject<T>(&mut self, error: RetainedSprDiagnostic) -> Result<T, RetainedSprDiagnostic> {
        self.error = Some(error);
        Err(error)
    }

    fn byte(&mut self, byte: u8) -> Result<(), RetainedSprDiagnostic> {
        match self.stage {
            Stage::Header => {
                self.header[self.consumed as usize - 1] = byte;
                if self.consumed == 32 { self.header_complete()?; }
            }
            Stage::Length => {
                self.frame_hash.update(&[byte]);
                if Self::varint_byte(&mut self.body_len, &mut self.length_bytes, byte)? {
                    if self.body_len < 2 { return Err(RetainedSprDiagnostic::Frame); }
                    if self.body_len > self.limits.frame_body_bytes { return Err(RetainedSprDiagnostic::Capacity); }
                    let end = self.consumed.checked_add(self.body_len).and_then(|value| value.checked_add(8)).ok_or(RetainedSprDiagnostic::Capacity)?;
                    self.stage = if end > self.total { Stage::Torn } else { Stage::Body };
                }
            }
            Stage::Body => {
                self.crc.update_page(&[byte]); self.frame_hash.update(&[byte]);
                if self.body_read == 0 { self.kind = byte; }
                else if self.body_read == 1 {
                    self.flags = byte; self.raw_len_pending = byte & crate::wire::FRAME_FLAG_COMPRESSED != 0;
                    if self.kind == crate::REC_COMMIT && byte != crate::wire::FRAME_FLAG_CRITICAL { return Err(RetainedSprDiagnostic::Commit); }
                    if byte & !31 != 0 || (byte & 1 != 0) != (byte & 28 != 0) { return Err(RetainedSprDiagnostic::Frame); }
                } else {
                    if self.raw_len_pending && Self::varint_byte(&mut self.raw_len, &mut self.raw_len_bytes, byte)? { self.raw_len_pending = false; }
                    if self.kind == crate::REC_COMMIT && self.body_read < 66 { self.commit[self.body_read as usize - 2] = byte; }
                }
                self.body_read += 1;
                if self.body_read == self.body_len {
                    if self.raw_len_pending { return Err(RetainedSprDiagnostic::Frame); }
                    self.stage = Stage::Trailer;
                }
            }
            Stage::Trailer => {
                self.frame_hash.update(&[byte]); self.trailer[self.trailer_read] = byte; self.trailer_read += 1;
                if self.trailer_read == 8 { self.frame_complete()?; }
            }
            Stage::Torn => {}
            Stage::Done => return Err(RetainedSprDiagnostic::State),
        }
        Ok(())
    }

    fn header_complete(&mut self) -> Result<(), RetainedSprDiagnostic> {
        let header = &self.header;
        if header[..8] != super::MAGIC || u16::from_le_bytes([header[8], header[9]]) != 1
            || u16::from_le_bytes([header[10], header[11]]) != 0
            || u32::from_le_bytes(header[12..16].try_into().unwrap()) != crate::REQUIRED_HASH_CHAIN
            || header[24..32].iter().any(|byte| *byte != 0)
            || crate::codec::crc32c(&header[..20]) != u32::from_le_bytes(header[20..24].try_into().unwrap()) {
            return Err(RetainedSprDiagnostic::Header);
        }
        self.chain = *semio_framework_hash::hash(header).as_bytes(); self.pending_hash.update(&self.chain);
        self.stage = Stage::Length;
        Ok(())
    }

    fn varint_byte(value: &mut u64, count: &mut u8, byte: u8) -> Result<bool, RetainedSprDiagnostic> {
        if *count == 9 && byte > 1 { return Err(RetainedSprDiagnostic::Frame); }
        *value |= u64::from(byte & 127) << (u32::from(*count) * 7);
        *count += 1;
        if byte < 128 {
            if *count > 1 && byte == 0 { return Err(RetainedSprDiagnostic::Frame); }
            return Ok(true);
        }
        Ok(false)
    }

    fn frame_complete(&mut self) -> Result<(), RetainedSprDiagnostic> {
        let frame_len = self.consumed - self.frame_start;
        if u32::from_le_bytes(self.trailer[..4].try_into().unwrap()) != self.crc.finish()
            || u64::from(u32::from_le_bytes(self.trailer[4..].try_into().unwrap())) != frame_len {
            return Err(RetainedSprDiagnostic::Frame);
        }
        self.frames = self.frames.checked_add(1).ok_or(RetainedSprDiagnostic::Capacity)?;
        if self.frames > self.limits.records { return Err(RetainedSprDiagnostic::Capacity); }
        if self.kind == crate::REC_COMMIT { self.commit_complete(frame_len)?; }
        else {
            self.pending_hash.update(self.frame_hash.finalize().as_bytes());
            self.pending_bytes = self.pending_bytes.checked_add(frame_len).ok_or(RetainedSprDiagnostic::Capacity)?;
            self.pending_records = self.pending_records.checked_add(1).ok_or(RetainedSprDiagnostic::Capacity)?;
        }
        self.frame_start = self.consumed; self.body_len = 0; self.body_read = 0; self.length_bytes = 0; self.trailer_read = 0;
        self.raw_len = 0; self.raw_len_bytes = 0; self.raw_len_pending = false;
        self.commit.fill(0); self.trailer.fill(0); self.crc = Crc32cCursor::new(); self.frame_hash = Hasher::new(); self.stage = Stage::Length;
        Ok(())
    }

    fn commit_complete(&mut self, frame_len: u64) -> Result<(), RetainedSprDiagnostic> {
        if self.flags != crate::wire::FRAME_FLAG_CRITICAL || self.body_len != 66 || frame_len != super::COMMIT_FRAME_LEN {
            return Err(RetainedSprDiagnostic::Commit);
        }
        let sequence = self.sequence.checked_add(1).ok_or(RetainedSprDiagnostic::Capacity)?;
        let payload = &self.commit; let chain = *self.pending_hash.finalize().as_bytes();
        if u64::from_le_bytes(payload[..8].try_into().unwrap()) != sequence
            || u64::from_le_bytes(payload[8..16].try_into().unwrap()) != self.commit_offset
            || u64::from_le_bytes(payload[16..24].try_into().unwrap()) != self.pending_bytes
            || u32::from_le_bytes(payload[24..28].try_into().unwrap()) != self.pending_records
            || payload[28..32] != [0; 4] || payload[32..] != chain {
            return Err(RetainedSprDiagnostic::Commit);
        }
        self.sequence = sequence; self.commit_offset = self.frame_start; self.verified_end = self.consumed;
        self.verified_frames = self.frames; self.chain = chain; self.pending_hash = Hasher::new(); self.pending_hash.update(&chain);
        self.pending_bytes = 0; self.pending_records = 0;
        Ok(())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
