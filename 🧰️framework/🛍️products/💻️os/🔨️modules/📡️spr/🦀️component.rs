//! 🎞️ Protocol facade: the single public entry point to the whole `protocol_*` crate family — the
//! binary op-log format layer (`protocol_core/format/history/materialize/io`) plus the command and
//! collaboration semantics layer added by the `INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING`
//! amendment (`protocol_command/causal/conflict/wire`) — `protocol_crdt` was deleted in favor of
//! `protocol_conflict`'s first-class quarantine/degrade model
//! (`.🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`).
//! Every downstream crate (`db`, `vcs`, app-layer
//! `#[derive(crate::os_dsl::DslOps)]` consumers) depends on `protocol`, never on the individual sub-crates
//! directly, so this file's re-export surface IS the family's frozen public API. Frozen contracts:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol (facade)` +
//! `### protocol (facade) — additional re-exports`).

//#region 🔖️Reexports
pub use crate::os_spr::format::{FrameCursor, RecordFrame, RecoveryMode, RecoveryReport, ReverseFrameCursor, SprWriter, VerificationLevel, WriteOptions};
pub use crate::os_spr::history::{
    AlternativeHead, DecodeOptions, EncodeOptions, FrontierComparison, FrontierSummary, HistoryAlternative, HistoryAppender, HistoryAuthor, HistoryChange, HistoryCheckpoint, HistoryComposition, HistoryCursor, HistoryEdit, HistoryLog, HistoryOpMeta,
    HistoryReader, OpPayload, REC_COMPOSITION, REC_CURSOR, decode_history, encode_history, frontier_delta, parse_ops_text, print_ops_text,
};
#[cfg(not(target_arch = "wasm32"))]
pub use crate::os_spr::io::{CompactOptions, HistoryFile, KeepSnapshots, ResumeState, TailFollower, compact, recover_file};
pub use crate::os_spr::materialize::{BaseBytes, BaseSnapshot, CheckpointPolicy, MaterializePlan, MaterializeReport, MaterializeTarget, SnapshotBodyKind, SnapshotRecord, materialize_with, resolve_plan};
pub use crate::os_spr::wire::{ProtocolError, ProtocolLimits, RecordHasher, SignatureVerifier, Signer};

pub use crate::os_spr::causal::{
    ArtifactDiff, FrontierComparison as RuntimeFrontierComparison, FrontierSummary as RuntimeFrontierSummary, InsertResult, InverseMutation, MutationDag, MutationDagError, MutationEnvelope, MutationTransform, TransformOutcome, decode_envelope,
    decode_envelopes, decode_frontier, decode_ops_vec, encode_envelope, encode_envelopes, encode_frontier, encode_ops_vec, frontier_delta as runtime_frontier_delta, mutation_envelope_from_edit, mutation_ids_for_edit,
};
pub use crate::os_spr::channel::{AppCommand, AppFrame, CHANNEL_VERSION, ChildPackEntry, decode_app_frame, encode_app_command, encode_app_frame};
pub use crate::os_spr::command::{
    APPROVED_VERBS, CollectionDiff, CollectionMutation, CommandOutcome, CompositeMutationKind, DiffAlgebra, DiffCodec, DiffRegions, Edit, ForeignStep, ForeignTarget, Identified, IndexedTripleDiff, Inference, InferenceFieldSpec, InferenceSpec,
    ItemPatch, MAX_PLAN_DEPTH, Mutation, MutationApplyError, MutationApplyResult, MutationDescriptor, MutationDiff, MutationEvent, MutationKind, MutationMessage, MutationMeta, MutationOrigin, MutationOutcome, MutationUpcaster, NamedTripleDiff,
    OpBinary, OpText, Patchable, PlanError, PlanStep, Planner, SemanticDescriptor, SemanticMutation, TouchedPaths, apply_collection_mutation, collection_diff_from_mutation, fold_plan_diff, fold_plan_inverse, indexed_apply,
    inverse_collection_mutation, is_approved_verb, mutation_descriptor, named_apply, plan_foreign_steps, plan_of, register_mutation_descriptor, str_eq, worst_level,
};
pub use crate::os_spr::conflict::{Conflict, ConflictId, ConflictKind, ConflictResolution, ConflictStatus, DispatchReport, EditMessages, MergeReport};
pub use crate::os_spr::wire::{
    AckStage, ApplyOutcome, Bootstrap, ClientFrame, Lane, PresencePeer, PresenceUi, PresenceViewKind, PresenceWindowView, ServerFrame, decode_client_frame, decode_presence_peer, decode_server_frame, encode_client_frame, encode_presence_peer,
    encode_server_frame,
};
pub use crate::os_spr::wire::{
    ActorId, ArtifactId, ArtifactVersion, HybridLogicalTimestamp, MergePolicy, MutationId, PayloadHash, SchemaId, SchemaVersion, StateClass, UndoPolicy, read_f64, read_str, read_varint_u64, write_f64, write_str, write_varint_u64,
};
//#endregion 🔖️Reexports

//#region 🔖️Compile
/// 🎬️ Ops text -> `.spr` binary, the bidirectional law `protocol_cli compile`/`decompile` exercise.
pub async fn compile_ops(ops: &str, options: &EncodeOptions) -> Result<Vec<u8>, ProtocolError> {
    encode_history(&parse_ops_text(ops)?, options).await
}

/// 🎬️ `.spr` binary -> ops text, the inverse of `compile_ops`.
pub async fn decompile_ops(bytes: &[u8], options: &DecodeOptions) -> Result<String, ProtocolError> {
    print_ops_text(&decode_history(bytes, options).await?)
}
//#endregion 🔖️Compile

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
        return Err(ProtocolError::Malformed { what: "extract_range ordinals", offset: 0, detail: "range must be non-empty (start < end)".to_string() });
    }
    let recovery = crate::os_spr::format::recover(&protocol_bytes, &ProtocolLimits::default(), RecoveryMode::LastCommit).await?;
    let trusted = &protocol_bytes[..recovery.bytes_recovered as usize];

    let mut cursor = FrameCursor::new(trusted, crate::os_spr::format::HEADER_SIZE as u64).await;
    let mut ordinal = 0u64;
    let mut start_offset: Option<u64> = None;
    let mut end_offset: Option<u64> = None;
    while let Some(frame) = cursor.next_frame().await? {
        if frame.kind == crate::os_spr::REC_EDIT {
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
    if &computed == expected_chain { Ok(()) } else { Err(ProtocolError::Malformed { what: "record slice chain", offset: 0, detail: "computed content chain does not match expected_chain".to_string() }) }
}

/// 🔐️ Shared by `verify_slice` and this crate's own tests: folds every frame's `blake3(full frame
/// bytes)` digest in `slice` into one combined digest, in frame order.
async fn slice_content_chain(slice: &[u8]) -> Result<[u8; 32], ProtocolError> {
    let hasher = crate::os_spr::format::Blake3Hasher;
    let mut cursor = FrameCursor::new(slice, 0).await;
    let mut concat = Vec::new();
    while let Some(frame) = cursor.next_frame().await? {
        let frame_bytes = &slice[frame.offset as usize..(frame.offset + frame.frame_len().await) as usize];
        concat.extend_from_slice(&hasher.hash(frame_bytes));
    }
    Ok(hasher.hash(&concat))
}

/// 🧭️ Decodes just enough of a `.spr` file (trusted prefix + a full `HistoryLog` decode) to report
/// its current sync-relevant frontier: document identity, the latest edit, every alternative's
/// head, and the commit chain's current tip.
///
/// 🎯️ Design choice: `AlternativeHead.checkpoint_id` picks the LAST id in
/// `HistoryAlternative::checkpoint_ids` (append-only list, so its tail is the most recent
/// checkpoint); `head_edit_ordinal` for that alternative is the highest edit ordinal transitively
/// reachable through that checkpoint's `change_ids -> HistoryChange::edit_ids`. An alternative with
/// no checkpoints yet, or a document with no edits yet, reports ordinal `0` / an empty edit id —
/// the contract does not specify empty-history behavior, so this crate picks the least-surprising
/// default (matching a fresh `HistoryAppender::begin` which has written zero edits).
pub async fn content_frontier(protocol_bytes: &[u8]) -> Result<FrontierSummary, ProtocolError> {
    let decode_options = DecodeOptions::default();
    let log = decode_history(protocol_bytes, &decode_options).await?;
    let recovery = crate::os_spr::format::recover(&protocol_bytes, &decode_options.limits, RecoveryMode::LastCommit).await?;

    let (head_edit_ordinal, head_edit_id) = match log.edits.last() {
        Some(edit) => ((log.edits.len() - 1) as u64, edit.id.clone()),
        None => (0, String::new()),
    };

    let chain_hash = if recovery.last_commit_seq == 0 {
        crate::os_spr::format::Blake3Hasher.hash(&protocol_bytes[..crate::os_spr::format::HEADER_SIZE])
    } else {
        let mut cursor = FrameCursor::new(protocol_bytes, recovery.last_commit_offset).await;
        let frame = cursor.next_frame().await?.ok_or_else(|| ProtocolError::Malformed { what: "commit frame", offset: recovery.last_commit_offset, detail: "expected a commit frame at the recovered offset".to_string() })?;
        crate::os_spr::format::parse_commit_payload(frame.payload().await).await?.chain_hash
    };

    let alternatives = log.alternatives.iter().map(|alternative| build_alternative_head(&log, alternative)).collect();

    Ok(FrontierSummary { document_id: log.doc_id, head_edit_ordinal, head_edit_id, alternatives, last_commit_seq: recovery.last_commit_seq, chain_hash })
}

/// 🧭️ See `content_frontier`'s design-choice note for the derivation this implements.
// 🚫️async: R9 pure accessor — I/O-free lookup over already-decoded in-memory data, whose only
// consumer is `Iterator::map`'s sync closure above.
fn build_alternative_head(log: &HistoryLog, alternative: &HistoryAlternative) -> AlternativeHead {
    let checkpoint_id = alternative.checkpoint_ids.last().cloned().unwrap_or_default();
    let head_edit_ordinal = log.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).map_or(0, |checkpoint| checkpoint_head_edit_ordinal(log, checkpoint));
    AlternativeHead { alternative_id: alternative.id.clone(), checkpoint_id, head_edit_ordinal }
}

/// 🧭️ Highest edit ordinal transitively reachable through `checkpoint.change_ids -> edit_ids`.
// 🚫️async: R9 pure accessor — I/O-free, only consumer is `Option::map_or`'s sync closure above.
fn checkpoint_head_edit_ordinal(log: &HistoryLog, checkpoint: &HistoryCheckpoint) -> u64 {
    let ordinal_of: std::collections::HashMap<&str, u64> = log.edits.iter().enumerate().map(|(ordinal, edit)| (edit.id.as_str(), ordinal as u64)).collect();
    checkpoint.change_ids.iter().filter_map(|change_id| log.changes.iter().find(|change| &change.id == change_id)).flat_map(|change| change.edit_ids.iter()).filter_map(|edit_id| ordinal_of.get(edit_id.as_str()).copied()).max().unwrap_or(0)
}
//#endregion 🔖️Sync

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ Builds a tiny in-memory `.spr` buffer with `edit_count` edits (ids `"e0".."eN"`), each in
    /// its own commit generation so multi-commit paths (`extract_range`, `content_frontier`) get
    /// exercised too. Returns the raw bytes.
    async fn build_history_bytes(doc_id: &str, schema: &str, edit_count: usize) -> Vec<u8> {
        let mut appender = HistoryAppender::begin(Vec::new(), doc_id, schema, &WriteOptions::default()).await.unwrap();
        for i in 0..edit_count {
            let edit = HistoryEdit {
                id: format!("e{i}"),
                actor: None,
                started_at: format!("2026-07-27T00:00:{i:02}Z"),
                finished_at: None,
                coalesce_key: None,
                description: None,
                ops: vec![OpPayload { text: Some(format!("op-{i}")), binary: None }],
                inverse: Vec::new(),
                meta: None,
            };
            appender.append_edit(&edit).await.unwrap();
            appender.commit().await.unwrap();
        }
        appender.into_sink().await
    }

    //#region 🔖️Reexports
    #[semio_framework_async_macros::async_test]
    async fn reexported_types_match_sibling_crate_shapes() {
        let limits = ProtocolLimits::default();
        assert!(limits.max_file_len > 0);
        let hlt = HybridLogicalTimestamp::new(1, 1000);
        assert_eq!(hlt.physical_ms, 1000);
        let _ = ActorId("actor-1".to_string());
        let _ = ArtifactId("doc-1".to_string());
    }
    //#endregion 🔖️Reexports

    //#region 🔖️Compile
    #[semio_framework_async_macros::async_test]
    async fn compile_ops_decompile_ops_round_trip() {
        let log = HistoryLog {
            doc_id: "doc-1".to_string(),
            schema: "schema-1".to_string(),
            edits: vec![HistoryEdit {
                id: "e0".to_string(),
                actor: Some("actor-1".to_string()),
                started_at: "2026-07-27T00:00:00Z".to_string(),
                finished_at: Some("2026-07-27T00:00:01Z".to_string()),
                coalesce_key: None,
                description: Some("first edit".to_string()),
                ops: vec![OpPayload { text: Some("set foo = 1".to_string()), binary: None }],
                inverse: Vec::new(),
                meta: None,
            }],
            changes: Vec::new(),
            checkpoints: Vec::new(),
            alternatives: Vec::new(),
            active_alternative_id: None,
            // 🎯️ W4: cursor is text-representable (unlike inverse, which is `.spr`-only) —
            // include one here to prove the compile_ops/decompile_ops text-tooling path preserves
            // it byte-for-byte, same as every other structural line.
            cursor: Some(HistoryCursor { applied_edit_ids: vec!["e0".to_string()], redo_edit_ids: Vec::new(), checkpoint_id: None }),
            composition: None,
            conflicts: Vec::new(),
        };
        let ops_text = print_ops_text(&log).unwrap();

        let compiled = compile_ops(&ops_text, &EncodeOptions::default()).await.unwrap();
        let decompiled = decompile_ops(&compiled, &DecodeOptions::default()).await.unwrap();

        assert_eq!(parse_ops_text(&decompiled).unwrap(), parse_ops_text(&ops_text).unwrap());
        assert_eq!(parse_ops_text(&decompiled).unwrap().cursor, log.cursor);
    }

    #[semio_framework_async_macros::async_test]
    async fn compile_ops_rejects_malformed_text() {
        assert!(compile_ops("not a valid ops line", &EncodeOptions::default()).await.is_err());
    }
    //#endregion 🔖️Compile

    //#region 🔖️Sync
    #[semio_framework_async_macros::async_test]
    async fn extract_range_returns_contiguous_slice_covering_requested_edits() {
        let bytes = build_history_bytes("doc-1", "schema-1", 4).await;

        // Independently compute each edit frame's [offset, offset+frame_len) span for cross-check.
        let mut cursor = FrameCursor::new(&bytes, crate::os_spr::format::HEADER_SIZE as u64).await;
        let mut edit_spans = Vec::new();
        while let Some(frame) = cursor.next_frame().await.unwrap() {
            if frame.kind == crate::os_spr::REC_EDIT {
                edit_spans.push((frame.offset, frame.offset + frame.frame_len().await));
            }
        }
        assert_eq!(edit_spans.len(), 4);

        let slice = extract_range(&bytes, 1..3).await.unwrap();
        assert_eq!(slice.first_edit_ordinal, 1);
        assert_eq!(slice.last_edit_ordinal, 2);
        assert_eq!(slice.count, 2);
        assert_eq!(slice.bytes, &bytes[edit_spans[1].0 as usize..edit_spans[2].1 as usize]);

        // The slice must itself be a structurally valid record stream (re-parseable from offset 0).
        let mut inner = FrameCursor::new(slice.bytes, 0).await;
        let mut edit_kinds_in_slice = 0;
        while let Some(frame) = inner.next_frame().await.unwrap() {
            if frame.kind == crate::os_spr::REC_EDIT {
                edit_kinds_in_slice += 1;
            }
        }
        assert_eq!(edit_kinds_in_slice, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn extract_range_rejects_empty_and_out_of_bounds_ranges() {
        let bytes = build_history_bytes("doc-1", "schema-1", 2).await;
        assert!(extract_range(&bytes, 1..1).await.is_err());
        assert!(extract_range(&bytes, 0..5).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn verify_slice_accepts_genuine_content_and_rejects_tamper() {
        let bytes = build_history_bytes("doc-1", "schema-1", 3).await;
        let slice = extract_range(&bytes, 0..2).await.unwrap();

        let expected = slice_content_chain(slice.bytes).await.unwrap();
        assert!(verify_slice(slice.bytes, &expected).await.is_ok());

        let mut tampered = slice.bytes.to_vec();
        tampered[0] ^= 0xFF;
        assert!(verify_slice(&tampered, &expected).await.is_err());

        assert!(verify_slice(slice.bytes, &[0u8; 32]).await.is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn content_frontier_reports_head_edit_and_chain_tip() {
        let bytes = build_history_bytes("doc-1", "schema-1", 3).await;
        let frontier = content_frontier(&bytes).await.unwrap();

        assert_eq!(frontier.document_id, "doc-1");
        assert_eq!(frontier.head_edit_ordinal, 2);
        assert_eq!(frontier.head_edit_id, "e2");
        assert_eq!(frontier.last_commit_seq, 3);
        assert!(frontier.alternatives.is_empty());

        // Cross-check chain_hash independently via the last REC_COMMIT frame's payload.
        let mut reverse = ReverseFrameCursor::at_end(&bytes[crate::os_spr::format::HEADER_SIZE..]).await;
        let last_commit = loop {
            let frame = reverse.prev_frame().await.unwrap().unwrap();
            if frame.kind == crate::os_spr::REC_COMMIT {
                break frame;
            }
        };
        let expected = crate::os_spr::format::parse_commit_payload(last_commit.payload().await).await.unwrap();
        assert_eq!(frontier.chain_hash, expected.chain_hash);
    }

    #[semio_framework_async_macros::async_test]
    async fn content_frontier_on_empty_history_reports_zero_head_and_no_alternatives() {
        let bytes = build_history_bytes("doc-1", "schema-1", 0);
        let frontier = content_frontier(&bytes.await).await.unwrap();
        assert_eq!(frontier.head_edit_ordinal, 0);
        assert_eq!(frontier.head_edit_id, "");
        assert_eq!(frontier.last_commit_seq, 0);
    }
    //#endregion 🔖️Sync
}
//#endregion 🧪️Tests
