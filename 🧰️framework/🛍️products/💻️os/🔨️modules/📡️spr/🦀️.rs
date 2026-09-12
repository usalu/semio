//! 🎞️ Protocol facade: the single public entry point to the whole `protocol_*` crate family — the
//! binary op-log format layer (`protocol_core/format/history/materialize/io`) plus the command and
//! collaboration semantics layer added by the `INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING`
//! amendment (`protocol_command/causal/conflict/wire`) — `protocol_crdt` was deleted in favor of
//! `protocol_conflict`'s first-class quarantine/degrade model
//! (`.🧬semio/🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`).
//! Every downstream crate (`db`, `vcs`, app-layer
//! `#[derive(crate::os_dsl::DslOps)]` consumers) depends on `protocol`, never on the individual sub-crates
//! directly, so this file's re-export surface IS the family's frozen public API. Frozen contracts:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol (facade)` +
//! `### protocol (facade) — additional re-exports`).

//#region 🔖️Reexports
pub use crate::os_spr::format::{FrameCursor, RecordFrame, RecoveryMode, RecoveryReport, ReverseFrameCursor, SprIdentityRecord, SprWriter, VerificationLevel, WriteOptions};
pub use crate::os_spr::format::retained::RetainedSprLimits;
pub use crate::os_spr::history::{
    decode_history, encode_history, frontier_delta, parse_ops_text, print_ops_text, AlternativeHead, DecodeOptions, EncodeOptions, FrontierComparison, FrontierSummary, HistoryAlternative, HistoryAppender, HistoryAuthor, HistoryChange,
    HistoryCheckpoint, HistoryComposition, HistoryCursor, HistoryEdit, HistoryLog, HistoryOpMeta, HistoryReader, OpPayload, RetainedHistoryDecode, RetainedHistoryDecodeStep, REC_COMPOSITION, REC_CURSOR,
};
#[cfg(not(target_arch = "wasm32"))]
pub use crate::os_spr::io::{compact, recover_file, CompactOptions, HistoryFile, KeepSnapshots, ResumeState, TailFollower};
pub use crate::os_spr::materialize::{materialize_with, resolve_plan, BaseBytes, BaseSnapshot, CheckpointPolicy, MaterializePlan, MaterializeReport, MaterializeTarget, SnapshotBodyKind, SnapshotRecord};
pub use crate::os_spr::wire::{ProtocolError, ProtocolLimits, RecordHasher, SignatureVerifier, Signer};

pub use crate::os_spr::causal::{
    decode_document_backbone_envelopes_exact, decode_document_backbone_envelopes_exact_with_limits, decode_envelope, decode_envelopes, decode_frontier, decode_ops_vec, encode_envelope, encode_envelopes, encode_frontier, encode_ops_vec,
    frontier_delta as runtime_frontier_delta, mutation_envelope_from_edit, mutation_ids_for_edit, ArtifactDiff, DocumentBackboneBatchLimitsV1, FrontierComparison as RuntimeFrontierComparison, FrontierSummary as RuntimeFrontierSummary, InsertResult,
    InverseMutation, MutationDag, MutationDagAppliedStep, MutationDagCloseOwner, MutationDagError, MutationDagInsertRejected, MutationDagSeedRejected, MutationEnvelope, MutationTransform, TransformOutcome, DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES,
    DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES, DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES,
};
pub use crate::os_spr::channel::{
    decode_app_frame, decode_document_archive_bytes, encode_app_command, encode_app_frame, encode_document_archive_bytes, encode_local_interaction_query_frame_into, AppCommand, AppFrame, ChildPackEntry, DecodedAppCommandOwner, DocumentArchiveArtifactRef,
    DocumentArchiveLoadState, DocumentArchiveLoadStatus, DocumentArchiveOwnerRef, DocumentArchivePack, OwnedDocumentMemberPackEntry, PagedAppCommandDecodeCursor, PresenceCommandCursor, WindowConfigPackEntry, CHANNEL_VERSION, DOCUMENT_ARCHIVE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS,
    INVOCATION_RESULT_PACK_MAXIMUM_BYTES,
};
pub use crate::os_spr::command::{
    apply_collection_mutation, collection_diff_from_mutation, fold_plan_diff, fold_plan_inverse, indexed_apply, inverse_collection_mutation, is_approved_verb, mutation_descriptor, named_apply, plan_foreign_steps, plan_of,
    register_mutation_descriptor, register_mutation_descriptors, str_eq, validate_mutation_leaf_descriptor, validate_mutation_leaf_descriptor_roster, validate_mutation_leaf_descriptor_roster_uniqueness, validate_mutation_leaf_source, worst_level,
    CollectionDiff, CollectionMutation, CommandOutcome, CompositeMutationKind, DiffAlgebra, DiffCodec, DiffRegions, Edit, ForeignStep, ForeignTarget, Identified, IndexedTripleDiff, Inference, InferenceFieldSpec, InferenceSpec, ItemPatch, Mutation,
    MutationApplyError, MutationApplyResult, MutationComposition, MutationDescriptor, MutationDescriptorError, MutationDescriptorRegistry, MutationDiff, MutationDiffParticipation, MutationDomainOperation, MutationEvent, MutationInvertibility,
    MutationKind, MutationLanguageSurface, MutationLeaf, MutationLeafDescriptor, MutationLeafDescriptorRosterValidationError, MutationLeafDescriptorValidationError, MutationLeafSourceScope, MutationLeafSourceValidationError, MutationMessage,
    MutationMeta, MutationOrigin, MutationOutcome, MutationOutcomeClass, MutationOwnerLayout, MutationSourceProvenance, MutationUpcaster, NamedTripleDiff, OpBinary, OpText, Patchable, PlanError, PlanStep, Planner, SemanticDescriptor,
    SemanticMutation, TouchedPaths, ValidatedMutationLeafSourceScope, APPROVED_VERBS, MAX_PLAN_DEPTH,
};
pub use crate::os_spr::conflict::{Conflict, ConflictId, ConflictKind, ConflictResolution, ConflictStatus, DispatchReport, EditMessages, MergeReport};
pub use crate::os_spr::wire::{
    decode_client_frame, decode_presence_peer, decode_server_frame, encode_client_frame, encode_presence_peer, encode_server_frame, AckStage, ApplyOutcome, Bootstrap, ClientFrame, Lane, PresencePeer, PresenceUi, PresenceViewKind, PresenceWindowView,
    ServerFrame,
};
pub use crate::os_spr::wire::{
    read_f64, read_str, read_varint_u64, write_f64, write_str, write_varint_u64, ActorId, ArtifactId, ArtifactVersion, HybridLogicalTimestamp, MergePolicy, MutationId, PayloadHash, SchemaId, SchemaVersion, StateClass, UndoPolicy,
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
    if &computed == expected_chain {
        Ok(())
    } else {
        Err(ProtocolError::Malformed { what: "record slice chain", offset: 0, detail: "computed content chain does not match expected_chain".to_string() })
    }
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
        crate::os_spr::format::parse_commit_payload(frame.payload().await)?.chain_hash
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
