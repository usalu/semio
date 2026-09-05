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
mod tests {
    use super::*;

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).unwrap() }

    fn hex(value: &str) -> Vec<u8> {
        value.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
    }

    async fn actual_bytes(fixture: &serde_json::Value) -> Vec<u8> {
        let options = super::super::WriteOptions { required_flags: crate::REQUIRED_HASH_CHAIN, optional_flags: crate::OPTIONAL_CANONICAL };
        let mut writer = super::super::SprWriter::begin(Vec::new(), &options).await.unwrap();
        for commit in fixture["commits"].as_array().unwrap() {
            for record in commit["records"].as_array().unwrap() {
                writer.write_record(record["kind"].as_u64().unwrap() as u8, true, &hex(record["payloadHex"].as_str().unwrap()), crate::codec::ids::CodecId(0)).await.unwrap();
            }
            assert_eq!(writer.commit().await.unwrap(), commit["offset"].as_u64().unwrap());
            assert_eq!(writer.position().await, commit["end"].as_u64().unwrap());
        }
        writer.into_sink().await
    }

    fn scan(bytes: &[u8], grant: usize, limits: RetainedSprLimits) -> Result<VerifiedSprSpan, RetainedSprDiagnostic> {
        let mut scan = RetainedSprVerification::new(bytes.len() as u64, limits)?;
        while scan.consumed() < bytes.len() as u64 {
            let start = scan.consumed() as usize; let mut fuel = grant;
            let read = scan.push(&bytes[start..], &mut fuel)?;
            assert!(read > 0 && read <= grant); assert_eq!(read + fuel, grant);
        }
        scan.finish()
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_spr_resume_preserves_exact_prefix_and_commit_chain() {
        let fixture = fixture();
        let bytes = actual_bytes(&fixture).await;
        let resume = &fixture["resume"];
        let record = &resume["record"];
        for cut in resume["cuts"].as_array().unwrap() {
            let cut = cut.as_u64().unwrap() as usize;
            let span = scan(&bytes[..cut], 7, RetainedSprLimits::default()).unwrap();
            let end = span.end() as usize;
            let sequence = span.sequence();
            let frames = span.frames();
            let previous_offset = span.commit_offset();
            let previous_chain = *span.chain();
            let mut writer = super::super::SprWriter::resume_verified(bytes[..end].to_vec(), span).await.unwrap();
            assert_eq!(writer.position().await, end as u64);
            writer.write_record(record["kind"].as_u64().unwrap() as u8, true, &hex(record["payloadHex"].as_str().unwrap()), crate::codec::ids::CodecId(0)).await.unwrap();
            let offset = writer.commit().await.unwrap() as usize;
            let resumed = writer.into_sink().await;
            assert_eq!(&resumed[..end], &bytes[..end]);
            assert_eq!(resumed.len() - end, resume["addedBytes"].as_u64().unwrap() as usize);
            let next = scan(&resumed, 1, RetainedSprLimits::default()).unwrap();
            assert_eq!(next.sequence(), sequence + 1);
            assert_eq!(next.frames(), frames + 2);
            assert_eq!(next.end(), resumed.len() as u64);
            assert_eq!(next.tail(), 0);
            assert_eq!(u64::from_le_bytes(resumed[offset + 11..offset + 19].try_into().unwrap()), previous_offset);
            let mut independent = blake3::Hasher::new();
            independent.update(&previous_chain);
            independent.update(blake3::hash(&resumed[end..offset]).as_bytes());
            assert_eq!(&resumed[offset + 35..offset + 67], independent.finalize().as_bytes());
            for delta in resume["wrongSinkOffsets"].as_array().unwrap() {
                let span = scan(&bytes[..cut], 7, RetainedSprLimits::default()).unwrap();
                let wrong_len = (span.end() as i64 + delta.as_i64().unwrap()) as usize;
                assert!(super::super::SprWriter::resume_verified(vec![0; wrong_len], span).await.is_err());
            }
        }
        let span = scan(&bytes, 7, RetainedSprLimits::default()).unwrap();
        let mut writer = super::super::SprWriter::resume_verified(bytes.clone(), span).await.unwrap();
        writer.next_commit_seq = resume["exhaustedSequence"].as_str().unwrap().parse().unwrap();
        assert!(writer.commit().await.is_err());
        assert_eq!(writer.into_sink().await, bytes);
        eprintln!("[DEBUG] SPR resume: 6 verified prefixes survive byte-exactly; chain/sequence/offset continue; 12 wrong sink lengths and exhausted sequence denied");
    }

    async fn verify_compressed_fixture(fixture: &serde_json::Value) {
        let header = hex(fixture["headerHex"].as_str().unwrap());
        let frame = |kind, flags, payload: &[u8]| {
            let mut body = vec![kind, flags]; body.extend_from_slice(payload);
            let mut bytes = Vec::new(); crate::codec::write_varint_u64(&mut bytes, body.len() as u64);
            bytes.extend_from_slice(&body); bytes.extend_from_slice(&crate::codec::crc32c(&body).to_le_bytes());
            let length = bytes.len() as u32 + 4; bytes.extend_from_slice(&length.to_le_bytes()); bytes
        };
        for row in fixture["compressed"].as_array().unwrap() {
            let kind = row["kind"].as_u64().unwrap() as u8; let flags = row["flags"].as_u64().unwrap() as u8;
            let mut payload = hex(row["rawLengthHex"].as_str().unwrap()); let stored = hex(row["storedHex"].as_str().unwrap()); payload.extend_from_slice(&stored);
            let record = frame(kind, flags, &payload);
            let mut chain = blake3::Hasher::new(); chain.update(blake3::hash(&header).as_bytes()); chain.update(blake3::hash(&record).as_bytes());
            let mut commit = [0u8; 64]; commit[..8].copy_from_slice(&1u64.to_le_bytes());
            commit[16..24].copy_from_slice(&(record.len() as u64).to_le_bytes()); commit[24..28].copy_from_slice(&1u32.to_le_bytes());
            commit[32..].copy_from_slice(chain.finalize().as_bytes());
            let mut bytes = header.clone(); bytes.extend_from_slice(&record); bytes.extend_from_slice(&frame(crate::REC_COMMIT, crate::wire::FRAME_FLAG_CRITICAL, &commit));
            for grant in [1, 7, 4096] {
                let result = scan(&bytes, grant, RetainedSprLimits::default());
                match row["error"].as_str() {
                    None => { let span = result.unwrap(); assert_eq!(span.end(), bytes.len() as u64); assert_eq!(span.sequence(), 1); assert_eq!(span.frames(), 2); assert_eq!(span.tail(), 0); }
                    Some("frame") => assert_eq!(result, Err(RetainedSprDiagnostic::Frame), "{}", row["id"]),
                    Some("commit") => assert_eq!(result, Err(RetainedSprDiagnostic::Commit), "{}", row["id"]),
                    _ => unreachable!(),
                }
            }
            if row["error"].is_null() {
                let raw = hex(row["rawHex"].as_str().unwrap()); let mut original = Vec::new();
                super::super::write_frame_retained(&mut original, kind, flags, Some(raw.len() as u64), &stored).await.unwrap();
                assert_eq!(original, record, "compressed framing differs from the production retained writer");
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_spr_verification_matches_neutral_commits_and_torn_prefixes() {
        let fixture = fixture(); let bytes = actual_bytes(&fixture).await;
        assert_eq!(&bytes[..32], hex(fixture["headerHex"].as_str().unwrap()));
        for grant in fixture["fuelGrants"].as_array().unwrap() {
            let grant = grant.as_u64().unwrap() as usize;
            for end in 32..=bytes.len() {
                let span = scan(&bytes[..end], grant, RetainedSprLimits::default()).unwrap();
                let commit = fixture["commits"].as_array().unwrap().iter().rev().find(|row| row["end"].as_u64().unwrap() <= end as u64);
                let committed = commit.map_or(32, |row| row["end"].as_u64().unwrap());
                assert_eq!(span.end(), committed); assert_eq!(span.tail(), end as u64 - committed);
                assert_eq!(span.sequence(), commit.map_or(0, |row| row["sequence"].as_u64().unwrap()));
                assert_eq!(span.frames(), commit.map_or(0, |row| row["recoveredFrames"].as_u64().unwrap()));
            }
        }
        let mut scan = RetainedSprVerification::new(bytes.len() as u64, RetainedSprLimits::default()).unwrap();
        assert_eq!(scan.push(&bytes, &mut 0), Ok(0)); assert_eq!(scan.consumed(), 0);
        assert_eq!(scan.finish(), Err(RetainedSprDiagnostic::State));
        let mut fuel = bytes.len(); scan.push(&bytes, &mut fuel).unwrap();
        let span = scan.finish().unwrap(); assert_eq!(span.end(), bytes.len() as u64);
        assert_eq!(scan.finish(), Err(RetainedSprDiagnostic::State));
        let mut chain = *blake3::hash(&bytes[..32]).as_bytes();
        for (start, record_ranges) in [(91, vec![(32, 75), (75, 91)]), (180, vec![(166, 180)])] {
            let mut hasher = blake3::Hasher::new(); hasher.update(&chain);
            for (from, to) in record_ranges { hasher.update(blake3::hash(&bytes[from..to]).as_bytes()); }
            chain = *hasher.finalize().as_bytes(); assert_eq!(&bytes[start + 35..start + 67], &chain);
        }
        assert_eq!(span.chain(), &chain);
        eprintln!("[DEBUG] retained SPR: 2 real writer commits, 224 LastCommit prefixes at 3 fuel grants; exact EOF required; one span handoff; no typed records");
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_spr_verification_rejects_hostile_frames_without_publication() {
        let fixture = fixture(); let bytes = actual_bytes(&fixture).await;
        verify_compressed_fixture(&fixture).await;
        for row in fixture["negative"].as_array().unwrap() {
            let mut mutated = bytes.clone(); let mut limits = RetainedSprLimits::default();
            match row["operation"].as_str().unwrap() {
                "replace-first-length" => { mutated.splice(32..33, hex(row["hex"].as_str().unwrap())); }
                "record-limit" => limits.records = row["value"].as_u64().unwrap(),
                "file-limit" => limits.file_bytes = row["value"].as_u64().unwrap(),
                operation => {
                    let offset = row["offset"].as_u64().unwrap() as usize + match operation { "commit-xor" => 94, "second-commit-xor" => 183, _ => 0 };
                    mutated[offset] ^= row["value"].as_u64().unwrap() as u8;
                    if row["repairCrc"].as_bool().unwrap() {
                        let (start, end) = match operation { "header-xor" => (0, 20), "second-commit-xor" => (181, 247), _ => (92, 158) };
                        let crc = crate::codec::crc32c(&mutated[start..end]); mutated[end..end + 4].copy_from_slice(&crc.to_le_bytes());
                    }
                }
            }
            let expected = match row["error"].as_str().unwrap() {
                "header" => RetainedSprDiagnostic::Header, "frame" => RetainedSprDiagnostic::Frame,
                "commit" => RetainedSprDiagnostic::Commit, "capacity" => RetainedSprDiagnostic::Capacity, _ => unreachable!(),
            };
            for grant in [1, 7, 4096] { assert_eq!(scan(&mutated, grant, limits), Err(expected), "{}", row["id"]); }
        }
        for boundary in 0..=bytes.len() {
            let mut scan = RetainedSprVerification::new(bytes.len() as u64, RetainedSprLimits::default()).unwrap();
            let mut fuel = boundary; scan.push(&bytes[..boundary], &mut fuel).unwrap(); scan.cancel();
            let position = scan.consumed(); let mut fuel = 4096;
            assert_eq!(scan.push(&bytes[boundary..], &mut fuel), Err(RetainedSprDiagnostic::Cancelled));
            assert_eq!(fuel, 4096); assert_eq!(scan.consumed(), position);
            assert_eq!(scan.finish(), Err(RetainedSprDiagnostic::Cancelled));
        }
        for end in 0..32 { assert_eq!(scan(&bytes[..end], 1, RetainedSprLimits::default()), Err(RetainedSprDiagnostic::Header)); }
        eprintln!("[DEBUG] retained SPR: 26 hostile header/frame/commit/limit denials and 10 compressed grammar cases at 3 grants; cancellation at all 256 byte boundaries; no input authority or semantic publication");
    }
}
