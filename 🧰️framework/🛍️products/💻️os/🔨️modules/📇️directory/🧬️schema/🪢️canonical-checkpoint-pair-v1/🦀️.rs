//! 🪢️ The hub's canonical checkpoint pair — the ONE seed every shell opens a hub document on
//! (`GET /spaces/{spaceId}/documents/{documentId}/active-checkpoint/pair`, media type
//! [`CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1`]). A door-created artifact's first checkpoint is the genesis its
//! owning component minted at creation; a checked-in document's is its latest Check In. The socket's `Welcome`
//! never carries a pair (its bootstrap is the database replay's `None`/`Tail`), so a cold shell fetches it.
//!
//! The Rust twin of `decodeCanonicalCheckpointPairV1`/`admitCanonicalCheckpointPairV1` in `../🟦️.ts` (same
//! refusal names), replayed by both from the language-agnostic fixture
//! `🧫️fixtures/📇️directory/🪢️canonical-checkpoint-pair-v1.json` (schema `🧬️schema/🪢️canonical-checkpoint-pair-v1`).
//! The exact inverse of the hub's `append_canonical_pair_{header,data,terminal}` (`🌎️hub/🛰️lag-rebootstrap`).

use super::{ArtifactFrontier, ArtifactHash, CheckpointId, DocumentOpenCheckpointV1, DocumentScope, PublishedArtifactBlob, DOCUMENT_OPEN_ID_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER};

pub const CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1: &str = "application/vnd.semio.canonical-checkpoint-pair.v1";
pub const CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES: usize = 16 * 1024;
pub const CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES: usize = 4 * 1024;
pub const CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS: usize = 16_384;
pub const CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES: usize = 64 * 1024 * 1024;
pub const CANONICAL_CHECKPOINT_PAIR_MAX_WIRE_BYTES: usize = CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES + CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES + CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS * 22 + 6;
const FORMAT_VERSION: u32 = 1;
const HEADER: u8 = 1;
const DATA: u8 = 2;
const TERMINAL: u8 = 3;
const PART_PACK: u8 = 1;
const PART_SPR: u8 = 2;
const TERMINAL_COMPLETE: u8 = 0;

/// 🚫️ Every way a pair body or its admission is refused, named exactly as the TypeScript twin throws it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalCheckpointPairRefusalV1 {
    Oversized,
    Truncated,
    FrameLength,
    Header,
    OversizedText,
    Text,
    Capacity,
    ZeroDigest,
    HeaderTrailingBytes,
    BaselineFrontier,
    PairLength,
    RecordCount,
    RecordTag,
    RecordOrdinal,
    RecordLength,
    RecordTrailingBytes,
    RecordOffset,
    RecordPart,
    Terminal,
    Incomplete,
    PackDigest,
    SprDigest,
    AggregateDigest,
    Scope,
    Checkpoint,
    Descriptor,
    Baseline,
    Aggregate,
}

impl CanonicalCheckpointPairRefusalV1 {
    /// 🏷️ The refusal's one spelling on every implementation.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Oversized => "canonical-checkpoint-pair.oversized",
            Self::Truncated => "canonical-checkpoint-pair.truncated",
            Self::FrameLength => "canonical-checkpoint-pair.frame-length",
            Self::Header => "canonical-checkpoint-pair.header",
            Self::OversizedText => "canonical-checkpoint-pair.oversized-text",
            Self::Text => "canonical-checkpoint-pair.text",
            Self::Capacity => "canonical-checkpoint-pair.capacity",
            Self::ZeroDigest => "canonical-checkpoint-pair.zero-digest",
            Self::HeaderTrailingBytes => "canonical-checkpoint-pair.header-trailing-bytes",
            Self::BaselineFrontier => "canonical-checkpoint-pair.baseline-frontier",
            Self::PairLength => "canonical-checkpoint-pair.pair-length",
            Self::RecordCount => "canonical-checkpoint-pair.record-count",
            Self::RecordTag => "canonical-checkpoint-pair.record-tag",
            Self::RecordOrdinal => "canonical-checkpoint-pair.record-ordinal",
            Self::RecordLength => "canonical-checkpoint-pair.record-length",
            Self::RecordTrailingBytes => "canonical-checkpoint-pair.record-trailing-bytes",
            Self::RecordOffset => "canonical-checkpoint-pair.record-offset",
            Self::RecordPart => "canonical-checkpoint-pair.record-part",
            Self::Terminal => "canonical-checkpoint-pair.terminal",
            Self::Incomplete => "canonical-checkpoint-pair.incomplete",
            Self::PackDigest => "canonical-checkpoint-pair.pack-digest",
            Self::SprDigest => "canonical-checkpoint-pair.spr-digest",
            Self::AggregateDigest => "canonical-checkpoint-pair.aggregate-digest",
            Self::Scope => "canonical-checkpoint-pair.scope",
            Self::Checkpoint => "canonical-checkpoint-pair.checkpoint",
            Self::Descriptor => "canonical-checkpoint-pair.descriptor",
            Self::Baseline => "canonical-checkpoint-pair.baseline",
            Self::Aggregate => "canonical-checkpoint-pair.aggregate",
        }
    }
}

impl std::fmt::Display for CanonicalCheckpointPairRefusalV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for CanonicalCheckpointPairRefusalV1 {}

/// 🪢️ One decoded canonical pair whose pack, SPR and aggregate digests match its own selection header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalCheckpointPairV1 {
    pub scope: DocumentScope,
    pub descriptor_digest_v1: ArtifactHash,
    pub active_checkpoint_id: CheckpointId,
    pub baseline_frontier: ArtifactFrontier,
    pub pack: PublishedArtifactBlob,
    pub spr: PublishedArtifactBlob,
    pub aggregate_sha256: ArtifactHash,
    pub pack_bytes: Vec<u8>,
    pub spr_bytes: Vec<u8>,
}

impl CanonicalCheckpointPairV1 {
    /// 🎫️ Admits the pair only as exactly the checkpoint the hub authorized for this open (the execution-target
    /// lease's or the open plan's `checkpoint`): a checkpoint that moved, a changed descriptor or a foreign scope
    /// is refused, never mounted.
    pub fn admit(&self, scope: &DocumentScope, expected: &DocumentOpenCheckpointV1) -> Result<(), CanonicalCheckpointPairRefusalV1> {
        if self.scope != *scope {
            return Err(CanonicalCheckpointPairRefusalV1::Scope);
        }
        if self.active_checkpoint_id.hex() != expected.checkpoint_id {
            return Err(CanonicalCheckpointPairRefusalV1::Checkpoint);
        }
        if self.descriptor_digest_v1.hex() != expected.descriptor_digest_v1 {
            return Err(CanonicalCheckpointPairRefusalV1::Descriptor);
        }
        if self.baseline_frontier != expected.baseline_frontier {
            return Err(CanonicalCheckpointPairRefusalV1::Baseline);
        }
        if self.aggregate_sha256.hex() != expected.aggregate_sha256 {
            return Err(CanonicalCheckpointPairRefusalV1::Aggregate);
        }
        Ok(())
    }
}

struct PairCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> PairCursor<'a> {
    fn exhausted(&self) -> bool {
        self.offset == self.bytes.len()
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], CanonicalCheckpointPairRefusalV1> {
        let end = self.offset.checked_add(length).filter(|end| *end <= self.bytes.len()).ok_or(CanonicalCheckpointPairRefusalV1::Truncated)?;
        let value = &self.bytes[self.offset..end];
        self.offset = end;
        Ok(value)
    }

    fn byte(&mut self) -> Result<u8, CanonicalCheckpointPairRefusalV1> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, CanonicalCheckpointPairRefusalV1> {
        let mut value = [0; 4];
        value.copy_from_slice(self.take(4)?);
        Ok(u32::from_be_bytes(value))
    }

    fn u64(&mut self) -> Result<u64, CanonicalCheckpointPairRefusalV1> {
        let mut value = [0; 8];
        value.copy_from_slice(self.take(8)?);
        let value = u64::from_be_bytes(value);
        if value > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(CanonicalCheckpointPairRefusalV1::Capacity);
        }
        Ok(value)
    }

    fn hash(&mut self) -> Result<ArtifactHash, CanonicalCheckpointPairRefusalV1> {
        let mut value = [0; 32];
        value.copy_from_slice(self.take(32)?);
        Ok(ArtifactHash(value))
    }

    fn digest(&mut self) -> Result<ArtifactHash, CanonicalCheckpointPairRefusalV1> {
        let hash = self.hash()?;
        if hash.0 == [0; 32] {
            return Err(CanonicalCheckpointPairRefusalV1::ZeroDigest);
        }
        Ok(hash)
    }

    fn text(&mut self, maximum: usize) -> Result<String, CanonicalCheckpointPairRefusalV1> {
        let length = self.u32()? as usize;
        if length > maximum {
            return Err(CanonicalCheckpointPairRefusalV1::OversizedText);
        }
        std::str::from_utf8(self.take(length)?).map(str::to_owned).map_err(|_| CanonicalCheckpointPairRefusalV1::Text)
    }

    fn frame(&mut self, maximum: usize) -> Result<PairCursor<'a>, CanonicalCheckpointPairRefusalV1> {
        let length = self.u32()? as usize;
        if length == 0 || length > maximum {
            return Err(CanonicalCheckpointPairRefusalV1::FrameLength);
        }
        Ok(PairCursor { bytes: self.take(length)?, offset: 0 })
    }
}

/// 🧮️ Exact record count the hub emits for a pair of these two lengths — pack records first, then SPR records,
/// each [`CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES`] except the last of each part.
pub const fn canonical_checkpoint_pair_record_count(pack_length: usize, spr_length: usize) -> usize {
    pack_length.div_ceil(CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES) + spr_length.div_ceil(CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES)
}

/// 🪢️ Decodes the hub's canonical-checkpoint-pair body and verifies its pack, SPR and aggregate
/// (`SHA-256(pack ‖ spr)`) digests against its own selection header: a stream of `u32be length | payload`
/// frames whose first payload is the selection header, whose middle payloads carry strictly ordered
/// pack-then-SPR data records at contiguous offsets, and whose last payload is the `Complete` terminal. Every
/// refusal is named; a partial, reordered, over-long, unterminated or tampered body is never half-accepted.
pub fn decode_canonical_checkpoint_pair_v1(input: &[u8]) -> Result<CanonicalCheckpointPairV1, CanonicalCheckpointPairRefusalV1> {
    if input.len() > CANONICAL_CHECKPOINT_PAIR_MAX_WIRE_BYTES {
        return Err(CanonicalCheckpointPairRefusalV1::Oversized);
    }
    let mut stream = PairCursor { bytes: input, offset: 0 };
    let mut header = stream.frame(CANONICAL_CHECKPOINT_PAIR_HEADER_MAX_BYTES)?;
    if header.byte()? != HEADER || header.u32()? != FORMAT_VERSION {
        return Err(CanonicalCheckpointPairRefusalV1::Header);
    }
    let scope = DocumentScope::new(header.text(DOCUMENT_OPEN_ID_MAX_BYTES)?, header.text(DOCUMENT_OPEN_ID_MAX_BYTES)?);
    let descriptor_digest_v1 = header.digest()?;
    let active_checkpoint_id = header.digest()?;
    let baseline_frontier = ArtifactFrontier { document_id: header.text(DOCUMENT_OPEN_ID_MAX_BYTES)?, head_edit_ordinal: header.u64()?, head_edit_id: header.text(DOCUMENT_OPEN_ID_MAX_BYTES)?, last_commit_seq: header.u64()?, chain_hash: header.hash()? };
    let pack = PublishedArtifactBlob { sha256: header.digest()?, byte_length: header.u64()? };
    let spr = PublishedArtifactBlob { sha256: header.digest()?, byte_length: header.u64()? };
    let aggregate_sha256 = header.digest()?;
    if !header.exhausted() {
        return Err(CanonicalCheckpointPairRefusalV1::HeaderTrailingBytes);
    }
    if !baseline_frontier.is_genesis_for(&scope) && !baseline_frontier.is_edited_for(&scope) {
        return Err(CanonicalCheckpointPairRefusalV1::BaselineFrontier);
    }
    let (pack_length, spr_length) = (pack.byte_length as usize, spr.byte_length as usize);
    if pack_length == 0 || spr_length == 0 || pack_length.saturating_add(spr_length) > CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES {
        return Err(CanonicalCheckpointPairRefusalV1::PairLength);
    }
    let records = canonical_checkpoint_pair_record_count(pack_length, spr_length);
    if records > CANONICAL_CHECKPOINT_PAIR_MAX_RECORDS {
        return Err(CanonicalCheckpointPairRefusalV1::RecordCount);
    }
    let mut pack_bytes = Vec::with_capacity(pack_length);
    let mut spr_bytes = Vec::with_capacity(spr_length);
    for ordinal in 0..records {
        let mut record = stream.frame(CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES + 18)?;
        if record.byte()? != DATA {
            return Err(CanonicalCheckpointPairRefusalV1::RecordTag);
        }
        let part = record.byte()?;
        if record.u32()? as usize != ordinal {
            return Err(CanonicalCheckpointPairRefusalV1::RecordOrdinal);
        }
        let offset = record.u64()? as usize;
        let length = record.u32()? as usize;
        if length == 0 || length > CANONICAL_CHECKPOINT_PAIR_RECORD_BYTES {
            return Err(CanonicalCheckpointPairRefusalV1::RecordLength);
        }
        let bytes = record.take(length)?;
        if !record.exhausted() {
            return Err(CanonicalCheckpointPairRefusalV1::RecordTrailingBytes);
        }
        let (target, declared) = match part {
            PART_PACK if pack_bytes.len() < pack_length => (&mut pack_bytes, pack_length),
            PART_SPR if pack_bytes.len() == pack_length => (&mut spr_bytes, spr_length),
            _ => return Err(CanonicalCheckpointPairRefusalV1::RecordPart),
        };
        if offset != target.len() || target.len() + length > declared {
            return Err(CanonicalCheckpointPairRefusalV1::RecordOffset);
        }
        target.extend_from_slice(bytes);
    }
    let mut terminal = stream.frame(2)?;
    if terminal.byte()? != TERMINAL || terminal.byte()? != TERMINAL_COMPLETE {
        return Err(CanonicalCheckpointPairRefusalV1::Terminal);
    }
    if !stream.exhausted() || pack_bytes.len() != pack_length || spr_bytes.len() != spr_length {
        return Err(CanonicalCheckpointPairRefusalV1::Incomplete);
    }
    if semio_framework_hash::Sha256::digest(&pack_bytes) != pack.sha256.0 {
        return Err(CanonicalCheckpointPairRefusalV1::PackDigest);
    }
    if semio_framework_hash::Sha256::digest(&spr_bytes) != spr.sha256.0 {
        return Err(CanonicalCheckpointPairRefusalV1::SprDigest);
    }
    let mut aggregate = semio_framework_hash::Sha256::new();
    aggregate.update(&pack_bytes);
    aggregate.update(&spr_bytes);
    if aggregate.finalize() != aggregate_sha256.0 {
        return Err(CanonicalCheckpointPairRefusalV1::AggregateDigest);
    }
    Ok(CanonicalCheckpointPairV1 { scope, descriptor_digest_v1, active_checkpoint_id, baseline_frontier, pack, spr, aggregate_sha256, pack_bytes, spr_bytes })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
