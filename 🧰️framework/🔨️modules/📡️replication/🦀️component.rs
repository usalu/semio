//! 📡️ Replication facade: the product-neutral wire contract shared by every replica and every
//! authority — frames, causal envelopes, the mutation contract, conflict vocabulary, the `.spr`
//! record format, and the pack codec primitives underneath them. The optimistic local replica
//! (os) and the authoritative server both speak exactly these bytes; neither owns them.
//!
//! Byte layout is frozen and cross-language: `🧫️fixtures/wire/` holds the 20 canonical frames that
//! the Rust and TypeScript codecs must both reproduce byte-identically.

use crate::format::{Blake3Hasher, FrameCursor, RecoveryMode, SprWriter};
use crate::wire::{ProtocolError, ProtocolLimits, RecordHasher};

//#region 🔖️Sync
/// 🔗️ Zero-copy: one contiguous borrowed byte span of whole record frames covering an edit-ordinal
/// range — itself a valid record stream, shippable as-is in a binary backbone/semio_hub frame.
pub struct RecordSlice<'a> {
    pub bytes: &'a [u8],
    pub first_edit_ordinal: u64,
    pub last_edit_ordinal: u64,
    pub count: u64,
}

/// 🔗️ Extracts the minimal contiguous byte span (over the file's trusted, recovered prefix) that
/// starts at the first `REC_EDIT` frame with ordinal `ordinals.start` and ends right after the
/// `REC_EDIT` frame with ordinal `ordinals.end - 1`. Any non-edit frames physically interleaved
/// between those two edits (dictionary deltas, commits, ...) are included verbatim since the
/// result must stay a byte-exact, re-parseable record stream; frames strictly before the first
/// target edit (e.g. an earlier dictionary base) are NOT included — a recipient shipping a slice
/// over the wire is assumed to already hold that earlier context (this crate's own choice, the
/// contract leaves exact slice bounds unspecified).
pub async fn extract_range<'a>(protocol_bytes: &'a [u8], ordinals: std::ops::Range<u64>) -> Result<RecordSlice<'a>, ProtocolError> {
    if ordinals.start >= ordinals.end {
        return Err(ProtocolError::Malformed { what: "extract_range ordinals", offset: 0, detail: "range must be non-empty (start < end).await".to_string() });
    }
    let recovery = crate::format::recover(&protocol_bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).await?;
    let trusted = &protocol_bytes[..recovery.bytes_recovered as usize];

    let mut cursor = FrameCursor::new(trusted, crate::format::HEADER_SIZE as u64).await;
    let mut ordinal = 0u64;
    let mut start_offset: Option<u64> = None;
    let mut end_offset: Option<u64> = None;
    while let Some(frame) = cursor.next_frame().await? {
        if frame.kind == crate::REC_EDIT {
            if start_offset.is_none() && ordinal >= ordinals.start {
                start_offset = Some(frame.offset);
            }
            if ordinal == ordinals.end - 1 {
                end_offset = Some(frame.offset + frame.frame_len().await);
                break;
            }
            ordinal += 1;
        }
    }

    let (start, end) = match (start_offset, end_offset) {
        (Some(s), Some(e)) => (s, e),
        _ => return Err(ProtocolError::Malformed { what: "extract_range ordinals", offset: 0, detail: format!("requested range {}..{} exceeds the file's {ordinal} recovered edits", ordinals.start, ordinals.end) }),
    };
    Ok(RecordSlice { bytes: &trusted[start as usize..end as usize], first_edit_ordinal: ordinals.start, last_edit_ordinal: ordinals.end - 1, count: ordinals.end - ordinals.start })
}

/// 🔐️ Content-integrity check for a `RecordSlice`'s bytes against a caller-supplied expected digest.
///
/// 🎯️ Design choice: the contract does not pin an exact algorithm for a slice-level chain (the
/// commit-chain algorithm in `protocol_format` is rooted in a specific prior commit's
/// `chain_hash`, which a mid-stream `RecordSlice` does not carry). This crate reuses that same
/// `digest_i = blake3(full frame bytes)` primitive, folding every frame's digest in the slice into
/// one `blake3(digest_1 || .. || digest_k)` value — i.e. the same shape as a commit's chain_hash,
/// but rooted at nothing (no `chain_{n-1}` prefix) since a slice is deliberately position-agnostic.
/// A caller (e.g. a semio_hub relaying a `RecordSlice`) computes this once at the source and ships the
/// digest alongside the bytes; the receiver calls `verify_slice` to detect any in-transit tamper.
pub async fn verify_slice(slice: &[u8], expected_chain: &[u8; 32]) -> Result<(), ProtocolError> {
    let computed = slice_content_chain(slice).await?;
    if &computed == expected_chain {
        Ok(())
    } else {
        Err(ProtocolError::Malformed { what: "record slice chain", offset: 0, detail: "computed content chain does not match expected_chain".to_string() })
    }
}

/// 🔐️ Shared by `verify_slice` and this crate's own tests: folds every frame's `blake3(full frame
/// bytes)` digest in `slice` into one combined digest, in frame order.
async fn slice_content_chain(slice: &[u8]) -> Result<[u8; 32], ProtocolError> {
    let hasher = crate::format::Blake3Hasher;
    let mut cursor = FrameCursor::new(slice, 0).await;
    let mut concat = Vec::new();
    while let Some(frame) = cursor.next_frame().await? {
        let frame_bytes = &slice[frame.offset as usize..(frame.offset + frame.frame_len().await) as usize];
        concat.extend_from_slice(&hasher.hash(frame_bytes));
    }
    Ok(hasher.hash(&concat))
}
//#endregion 🔖️Sync
