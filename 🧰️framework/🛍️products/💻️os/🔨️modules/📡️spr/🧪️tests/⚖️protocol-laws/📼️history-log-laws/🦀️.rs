//! 📼️ History log laws of the protocol laws: the log writer and reader helpers, the encode and decode identity, canonical stability, zero-copy, tamper detection, recovery truncation and compaction laws. See [`super`].

use super::*;

//#region 🔖️Laws
// `crate::os_spr::history::encode_history`/`decode_history`/`parse_ops_text`/`print_ops_text` are the
// whole-batch codec and text-grammar twin the frozen contract's laws are phrased against; the
// `protocol` facade doesn't re-export them (only `HistoryAppender`/`HistoryReader` and the
// `compile_ops`/`decompile_ops` ops-text wrappers), so this crate depends on `protocol_history`
// directly for the struct-level and text-level laws below (see `Cargo.toml`'s note).

/// 🖇️ `.spr` bytes for `log`: `commit_after_every_record: false` uses the whole-batch
/// `crate::os_spr::history::encode_history` codec directly (buffered); `true` streams it through
/// `crate::os_spr::HistoryAppender`, committing after every single record (the hot-path shape a live
/// appender actually uses) — see `assert_streamed_equals_buffered`.
pub async fn write_history_log(log: &crate::HistoryLog, commit_after_every_record: bool) -> Vec<u8> {
    if !commit_after_every_record {
        return crate::history::encode_history(log, &crate::history::EncodeOptions::default()).await.expect("encode_history must succeed for a testkit-generated log");
    }
    let options = crate::WriteOptions { required_flags: 0, optional_flags: 0 };
    let mut appender = crate::HistoryAppender::begin(Vec::new(), &log.doc_id, &log.schema, &options).await.expect("HistoryAppender::begin must succeed for a well-formed doc_id/schema");
    appender.commit().await.expect("commit after REC_DOC");
    for edit in &log.edits {
        appender.append_edit(edit).await.expect("append_edit must succeed for a testkit-generated edit");
        appender.commit().await.expect("commit after edit");
    }
    for transition in &log.transitions {
        appender.append_transition(transition).await.expect("append_transition must succeed for a testkit-generated transition");
        appender.commit().await.expect("commit after transition");
    }
    appender.commit().await.expect("final commit");
    appender.into_sink().await
}

pub(super) async fn read_history_log(bytes: &[u8]) -> crate::HistoryLog {
    crate::history::decode_history(bytes, &crate::history::DecodeOptions::default()).await.expect("decode_history must succeed for freshly-written bytes")
}

/// ✅️ LAW: `decode_history(&encode_history(log, _)) == log`.
pub async fn assert_history_encode_decode_identity(log: &crate::HistoryLog) {
    let decoded = read_history_log(&write_history_log(log, false).await).await;
    assert_eq!(&decoded, log, "encode_history/decode_history round trip diverged from the source HistoryLog");
}

/// ✅️ LAW: `encode_history` is byte-for-byte deterministic — encoding the same `HistoryLog` twice
/// produces identical bytes.
pub async fn assert_history_canonical_stable(log: &crate::HistoryLog) {
    let a = write_history_log(log, false).await;
    let b = write_history_log(log, false).await;
    assert_eq!(a, b, "encode_history must be byte-identical across repeated calls (canonical determinism law)");
}

/// ✅️ LAW: `parse_ops_text -> encode -> decode -> print_ops_text` is a fixpoint — reprinting the
/// result of one full round trip through `parse_ops_text`/`encode_history`/`decode_history`/
/// `print_ops_text` must reproduce the same text on a second parse/print pass (comments/blank
/// lines normalize away on the FIRST parse, matching `protocol_history`'s own
/// `ops_text_is_a_fixpoint_under_reprint` precedent test).
pub async fn assert_ops_protocol_bidirectional(ops_text: &str) {
    let encode_options = crate::history::EncodeOptions::default();
    let decode_options = crate::history::DecodeOptions::default();

    let log = crate::history::parse_ops_text(ops_text).expect("parse_ops_text must succeed for well-formed ops text");
    let bytes = crate::history::encode_history(&log, &encode_options).await.expect("encode_history must succeed for a well-formed HistoryLog");
    let decoded = crate::history::decode_history(&bytes, &decode_options).await.expect("decode_history must succeed on encode_history's own output");
    let printed = crate::history::print_ops_text(&decoded).expect("print_ops_text must succeed for text-only generated ops");

    let reparsed = crate::history::parse_ops_text(&printed).expect("parse_ops_text must succeed on print_ops_text's own output");
    assert_eq!(crate::history::print_ops_text(&reparsed).unwrap(), printed, "print_ops_text(parse_ops_text(text)) must be a fixpoint under a second parse/print pass");
}

/// ✅️ LAW: streaming `log` through `crate::os_spr::HistoryAppender` one commit per record decodes
/// identically to a single-shot `crate::os_spr::history::encode_history` of the whole log.
pub async fn assert_streamed_equals_buffered(log: &crate::HistoryLog) {
    let buffered = read_history_log(&write_history_log(log, false).await).await;
    let streamed = read_history_log(&write_history_log(log, true).await).await;
    assert_eq!(buffered, streamed, "HistoryAppender-per-record streaming and encode_history(whole log) must decode identically");
}

/// ✅️ LAW: every `RecordFrame::payload()` returned while sweeping `bytes` borrows zero-copy from
/// `bytes` itself.
pub async fn assert_zero_copy(bytes: &[u8]) {
    let bounds = bytes.as_ptr_range();
    let mut cursor = crate::FrameCursor::new(bytes, crate::format::HEADER_SIZE as u64).await;
    while let Some(frame) = cursor.next_frame().await.expect("assert_zero_copy requires an already-structurally-valid record stream") {
        let payload_bounds = frame.payload().await.as_ptr_range();
        assert!(payload_bounds.start >= bounds.start && payload_bounds.end <= bounds.end, "RecordFrame::payload() at offset {} must borrow zero-copy from the input slice", frame.offset);
    }
}

/// 🚪️ Opens a `HistoryReader` and decodes its `HistoryLog`, sequenced explicitly: `reader.log()`
/// is async, so it cannot be chained through `Result::and_then`'s sync closure (R10 shape 1).
pub async fn open_and_log(bytes: &[u8], options: &crate::DecodeOptions) -> Result<crate::HistoryLog, crate::ProtocolError> {
    match crate::HistoryReader::open(bytes, options).await {
        Ok(reader) => reader.log().await,
        Err(error) => Err(error),
    }
}

/// ✅️ LAW (softened to match the family's actual recovery-then-verify layering, see below): a
/// single-byte flip anywhere in `bytes` at or after the header must never go unnoticed under Full
/// verification. The frozen contract's prose says "must error", but per-frame CRC-32C is checked
/// during `crate::os_spr::format::recover`'s own scan BEFORE `VerificationLevel::Full`'s chain-hash check
/// ever runs — `protocol_history`'s own precedent test (`history_full_verification_detects_tampering`
/// in `protocol/history/rs/lib.rs`) documents the same real outcome: a corrupted frame's own CRC
/// mismatch makes `recover` truncate the trusted range BEFORE it, so the decode either errors OR
/// silently comes back shorter than the original — never silently equal to it. This fn asserts
/// exactly that non-silent-corruption invariant, sampling `24` roughly-evenly-spaced byte offsets
/// rather than every byte (this crate's own `assert_recovery_truncates_to_commit` already covers an
/// exhaustive-if-requested sweep of the structurally-analogous truncation case).
pub async fn assert_chain_detects_tamper(bytes: &[u8]) {
    let options = crate::DecodeOptions { verification: crate::VerificationLevel::Full, limits: crate::ProtocolLimits::default() };
    let original = open_and_log(bytes, &options).await.expect("assert_chain_detects_tamper requires an already-decodable, untampered input");

    for position in sampled_positions(bytes.len(), crate::format::HEADER_SIZE, 24).await {
        let mut tampered = bytes.to_vec();
        tampered[position] ^= 0xFF;
        let result = open_and_log(&tampered, &options).await;
        let went_unnoticed = matches!(&result, Ok(decoded) if decoded == &original);
        assert!(!went_unnoticed, "tampering byte {position} under Full verification must not silently decode to the original untampered log");
    }
}

/// 📐️ Up to `cap` roughly-evenly-spaced indices from `[floor, total)` — shared sampling core for
/// this crate's own byte-position law sweeps (`pack_testkit`'s equivalent is private).
pub(super) async fn sampled_positions(total: usize, floor: usize, cap: usize) -> Vec<usize> {
    if total <= floor {
        return Vec::new();
    }
    let span = total - floor;
    if span <= cap {
        return (floor..total).collect();
    }
    let step = span as f64 / cap as f64;
    let mut positions: Vec<usize> = (0..cap).map(|i| floor + ((i as f64 * step) as usize).min(span - 1)).collect();
    positions.sort_unstable();
    positions.dedup();
    positions
}

/// 📐️ Up to `cap` roughly-evenly-spaced truncation lengths in `[0, total]`, always including `0`
/// and `total`.
pub(super) async fn sampled_lengths(total: usize, cap: usize) -> Vec<usize> {
    if total == 0 {
        return vec![0];
    }
    if total <= cap {
        return (0..=total).collect();
    }
    let step = total as f64 / cap as f64;
    let mut lengths: Vec<usize> = (0..cap).map(|i| ((i as f64 * step) as usize).min(total)).collect();
    lengths.push(total);
    lengths.sort_unstable();
    lengths.dedup();
    lengths
}

/// ✅️ LAW: truncating `bytes` at every sampled length (density per `level`) then running
/// `crate::os_spr::format::recover` (`RecoveryMode::LastCommit`) always yields a trusted prefix that ends
/// exactly at a real `REC_COMMIT` frame — or, if no commit survived the truncation, exactly at the
/// 32-byte header.
pub async fn assert_recovery_truncates_to_commit(bytes: &[u8], level: CorruptionLevel) {
    let limits = crate::ProtocolLimits::default();
    let cap = match level {
        CorruptionLevel::Exhaustive => bytes.len(),
        CorruptionLevel::Long => 128,
        CorruptionLevel::Quick => 16,
    };
    for len in sampled_lengths(bytes.len(), cap).await {
        let truncated = &bytes[..len];
        let Ok(recovery) = crate::format::recover(&truncated, &limits, crate::RecoveryMode::LastCommit).await else {
            continue;
        };
        if recovery.last_commit_seq == 0 {
            assert_eq!(recovery.bytes_recovered as usize, crate::format::HEADER_SIZE, "no commit recovered at truncation length {len} -> trusted prefix must be exactly the header");
            continue;
        }
        let trusted = &truncated[..recovery.bytes_recovered as usize];
        let mut cursor = crate::FrameCursor::new(trusted, recovery.last_commit_offset).await;
        let frame = cursor.next_frame().await.expect("the recovered trusted prefix must itself re-parse cleanly").expect("a frame must exist at the reported last_commit_offset");
        assert_eq!(frame.kind, crate::REC_COMMIT, "last_commit_offset must point at a REC_COMMIT frame (truncation length {len})");
        assert_eq!(frame.offset + frame.frame_len().await, recovery.bytes_recovered, "bytes_recovered must end exactly after the last trusted commit frame (truncation length {len})");
    }
}

/// 🧮️ The `(kind, payload bytes)` multiset of every structural record (`REC_DOC`/`REC_EDIT`/
/// `REC_TRANSITION`) in `bytes`'s trusted prefix —
/// deliberately excludes `REC_COMMIT` (chain metadata, expected to differ across a compaction that
/// restarts the commit chain) and dictionary/index/sealed/compaction/snapshot/ephemeral kinds
/// (physical layout compaction is explicitly allowed to rewrite).
#[cfg(not(target_arch = "wasm32"))]
pub(super) async fn structural_records(bytes: &[u8], limits: &crate::ProtocolLimits) -> std::collections::BTreeMap<(u8, Vec<u8>), usize> {
    let recovery = crate::format::recover(&bytes, limits, crate::RecoveryMode::LastCommit).await.expect("recover must succeed on an already-valid stream");
    let trusted = &bytes[..recovery.bytes_recovered as usize];
    let mut counts = std::collections::BTreeMap::new();
    let mut cursor = crate::FrameCursor::new(trusted, crate::format::HEADER_SIZE as u64).await;
    while let Some(frame) = cursor.next_frame().await.expect("trusted prefix must re-parse cleanly") {
        if matches!(frame.kind, crate::REC_DOC | crate::REC_EDIT | crate::REC_TRANSITION) {
            *counts.entry((frame.kind, frame.payload().await.to_vec())).or_insert(0) += 1;
        }
    }
    counts
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) async fn scratch_spr_path(label: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_nanos());
    std::env::temp_dir().join(format!("protocol-testkit-{label}-{}-{nanos}.spr", std::process::id()))
}

/// ✅️ LAW: `crate::os_spr::compact` is physically-rewriting-but-identity-preserving — the
/// `(kind, payload bytes)` multiset of every structural record is unchanged across a compaction
/// round trip. Native-only (`crate::os_spr::compact` is a file-based API): writes `bytes` to a scratch
/// temp file, compacts it in place, reads it back.
#[cfg(not(target_arch = "wasm32"))]
pub async fn assert_compaction_identity(bytes: &[u8]) {
    let limits = crate::ProtocolLimits::default();
    let before = structural_records(bytes, &limits).await;

    let path = scratch_spr_path("compact").await;
    std::fs::write(&path, bytes).expect("write scratch .spr file for compaction test");
    let options = crate::CompactOptions { drop_ephemeral: true, keep_snapshots: crate::KeepSnapshots::All };
    // 🎯️ `compact` rewrites the file in place, so it must be awaited BEFORE reading it back —
    // reading first would race the physical rewrite and observe the pre-compaction bytes.
    let token=semio_framework_async::CancelToken::root_now();let transport=semio_framework_os_pack_tools::control::admit_command_transport(limits.max_total_alloc,token.clone(),|_|{}).unwrap();let context=semio_framework_os_pack_tools::control::CommandContext::try_new(transport,limits.max_total_alloc,token,|_|{}).unwrap();
    crate::compact(&path, &options, &limits, &context).await.expect("crate::os_spr::compact must succeed on an already-valid stream");
    let compacted = std::fs::read(&path);
    let _ = std::fs::remove_file(&path);

    let after = structural_records(&compacted.expect("read back compacted .spr file"), &limits).await;
    assert_eq!(before, after, "compaction must preserve the (kind, payload bytes) multiset of every structural record");
}
//#endregion 🔖️Laws
