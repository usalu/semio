//! 🧾️ Document envelope materialization: genesis-to-envelope construction and the materialized views the codecs and
//! the engine build documents from.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use crate::os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisCodec};
use crate::os_spr::{HybridLogicalTimestamp, MutationId};
use crate::os_spr::{Edit, Mutation, OpBinary};
use std::collections::HashSet;
use crate::os_vcs::VcsError;
use crate::os_vcs::{
    ArtifactHistoryLedger, ArtifactVcs, Identified,
};
use super::store_schema::{ArtifactBackboneRef, ArtifactCursor, ArtifactCursorOwners, ArtifactEnvelope, ArtifactEnvelopeOwners};
#[cfg(target_arch = "wasm32")]
use super::serialization::{ArtifactCodecApplyFuture};
use crate::os_pack::pack_rt;
use super::ledger::{ArtifactEditMessageLedger, fold_envelope_history};
use super::ledger::ReplayProjection;
use super::replay::fold_effective_edit;
#[cfg(target_arch = "wasm32")]
use super::backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use super::backbone::local_storage_backbone_key;

// config_spec_* removed — ConfigSpec is UI (framework-core); avoids kernel↔core cycle
pub fn create_document_envelope<P, Mutation>(schema: &str, id: &str, initial_snapshot: P, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
where P: Clone + ArtifactGenesisCodec,
{
    create_document_envelope_from_genesis(schema, id, AdmittedArtifactGenesis::born(initial_snapshot).expect("artifact genesis birth encodes once"), backbone)
}

pub fn create_document_envelope_from_genesis<P, Mutation>(schema: &str, id: &str, genesis: AdmittedArtifactGenesis<P>, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
where P: Clone,
{
    create_document_envelope_from_genesis_owners(schema.into(), id.into(), genesis, backbone)
}

/// 📥️ Moves exact original metadata into the document constructor while preserving its native catalogs.
pub fn create_document_envelope_from_genesis_owners<P, Mutation>(schema: String, id: String, genesis: AdmittedArtifactGenesis<P>, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation>
where P: Clone,
{
    ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema,
        id,
        vcs: ArtifactVcs { genesis, edits: ArtifactHistoryLedger::new(), changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() },
        backbone,
        active_alternative_id: None,
        cursor: Some(ArtifactCursor::default()),
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::HistoryFoldIndex::new(),
        edit_messages: ArtifactEditMessageLedger::new(),
        conflicts: Vec::new(),
        transitions: Vec::new(),
        history_shape: crate::os_spr::HistoryShape::Document,
        open_transaction: None,
        viewer_checkpoint_id: None,
    })
}

/// 🪶 Moves exact original Genesis and metadata into an allocation-free document catalog.
pub fn empty_document_envelope_from_genesis_owners<P, Mutation>(schema: String, id: String, genesis: AdmittedArtifactGenesis<P>, backbone: Option<ArtifactBackboneRef>) -> ArtifactEnvelope<P, Mutation> {
    ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema, id,
        vcs: ArtifactVcs { genesis, edits: ArtifactHistoryLedger::empty(), changes: ArtifactHistoryLedger::empty(), checkpoints: ArtifactHistoryLedger::empty(), alternatives: ArtifactHistoryLedger::empty() },
        backbone, active_alternative_id: None,
        cursor: Some(ArtifactCursor::from_owners(ArtifactCursorOwners { applied_edit_ids: crate::os_vcs::HistoryPageStack::empty(), redo_edit_ids: crate::os_vcs::HistoryPageStack::empty(), checkpoint_id: None })),
        dialect: None, migrated_from: None, owner: None, lanes: protocol::HistoryFoldIndex::new(),
        edit_messages: ArtifactEditMessageLedger::empty(), conflicts: Vec::new(), transitions: Vec::new(), history_shape: crate::os_spr::HistoryShape::Document, open_transaction: None, viewer_checkpoint_id: None,
    })
}

pub async fn edit_ids_for_changes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, change_ids: &[String]) -> Vec<String>
where
    Mutation: Clone,
    P: Clone,
{
    let mut edit_ids = Vec::new();
    for change_id in change_ids {
        if let Some(change) = envelope.vcs.changes.iter().find(|entry| entry.id == *change_id) {
            edit_ids.extend(change.edit_ids.iter().cloned());
        }
    }
    edit_ids
}

/// 🔂️ Full raw fold of `initial_snapshot` over every `applied_edit_ids` edit's `forwards`.
/// `crate::os_spr::command::Mutation::reconcile` and its diagnostic-bag-returning twin
/// `materialize_document_snapshot_with_conflicts` are GONE
/// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6/C10) — concurrent-merge
/// arbitration is now `ArtifactStore::ingest_remote`/`resolve_conflict`'s job against
/// `📡️spr/⚔️conflict`'s first-class `Conflict`/`MergeReport`, not a post-materialization hook here.
pub async fn materialize_document_snapshot<P, Mutation, I, S>(envelope: &ArtifactEnvelopeOwners<P, Mutation>, applied_edit_ids: I) -> Result<P, VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P> + OpBinary,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let supersessions = fold_envelope_history(envelope)?.supersessions;
    let mut snapshot = ReplayProjection::<P, Mutation>::new(envelope.vcs.genesis.facts().snapshot().clone());
    for edit_id in applied_edit_ids {
        let edit_id = edit_id.as_ref();
        let edit = envelope.vcs.edits.find_near(|entry| entry.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.to_string()))?;
        fold_effective_edit(&mut snapshot, edit, &envelope.schema, &supersessions, usize::MAX);
    }
    Ok(snapshot.into_inner())
}

/// 🕰️ Single timestamp source for `Edit.started_at`/`Checkpoint.timestamp` — re-exported so
/// callers outside this crate (e.g. the framework session command log) stamp entries in the
/// exact same format.
pub fn now_iso() -> String {
    format!("{}", now_ms())
}

/// 🪪️ A Store's replica clock: its `actor` is fresh platform entropy per Store instance, the
/// identity every edit and operation id this Store mints is namespaced by. Refusing to construct
/// without entropy is deliberate — a constant replica makes two sessions' first edits collide.
pub(crate) fn fresh_replica_clock() -> HybridLogicalTimestamp {
    HybridLogicalTimestamp::new(crate::os_identity::entropy_u64().expect("artifact store replica identity requires platform entropy"), now_ms())
}

/// 🕰️ Captures the platform Unix milliseconds used by original Store edits and replica clocks.
pub fn now_ms() -> u64 {
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
    }
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now() as u64
    }
}

/// 🩺️ Defense in depth, NOT the fix for the id-domain discontinuity itself (see {@link
/// stamp_primary_operation_identity}, which is): every id this returns is about to become a
/// `Change.edit_ids` entry, so if one isn't backed by a real `envelope.vcs.edits` entry, a much
/// louder and more specific `debug_assert!` here beats discovering it two calls later as
/// `validate_durable_history`'s cryptic "invalid edit reference". Deliberately does NOT filter the
/// id out in release builds — silently shrinking the checkpoint would drop a real edit out of
/// history without a trace, exactly the failure mode `checkpoint_after_ingesting_a_remote_edit_
/// stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives`'s doc comment warns against.
pub(crate) async fn uncommitted_edit_ids<P, Mutation, I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I) -> Vec<String>
where
    Mutation: Clone,
    P: Clone,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let committed: HashSet<String> = envelope.vcs.changes.iter().flat_map(|change| change.edit_ids.iter().cloned()).collect();
    applied_edit_ids
        .into_iter()
        .map(|id| id.as_ref().to_string())
        .filter(|id| !committed.contains(id))
        .inspect(|id| debug_assert!(envelope.vcs.edits.iter().any(|edit| edit.id == *id), "uncommitted_edit_ids: {id} is applied but has no backing envelope.vcs.edits entry — the checkpoint about to reference it will fail validate_durable_history"))
        .collect()
}

/// 🪪️ For a SINGLE-op edit only, overrides its sole `mutation_meta[0].mutation_id` with the edit's
/// own real `id` — preserving a SINGLE global edit-id domain across the wire for the dominant case
/// (one atomic user action, one forward op). `replay_mutations` always pre-fills
/// `mutation_meta[i].mutation_id` (`mutation.mutation_id().unwrap_or_else(mint_mutation_id` of the
/// raw op bytes), so by the time an `Edit` exists it is never absent — a plain `is_none()` guard
/// would never fire. `mutation_id()` has exactly one implementor repo-wide (the
/// `crate::os_spr::Mutation<P>` trait's own `None` default — confirmed zero overrides), so op 0
/// always carries `mint_mutation_id`'s hash of the raw op bytes and its replica clock tick: an identity that shares
/// nothing with `edit.id` (`mint_edit_id`'s replica+sequence+full-fingerprint hash) by construction of
/// the two different formulas — not merely "sometimes absent". For a single-op edit, op 0 IS the
/// edit's entire wire identity (`crate::os_spr::mutation_ids_for_edit`'s only entry for it), so
/// overriding it to the edit's real id means `edit_from_operation_envelope` reconstructs an EXACT
/// id match on every receiving store, instead of a content-hash id that never existed locally and
/// cannot resolve a history transition naming it. MUST NOT extend to a genuine multi-op edit
/// (`forwards.len() > 1`): `ingest_remote` reconstructs one local single-forward `Edit` per WIRE op,
/// so giving op 0 the parent edit's bare id would make that receiver's one-forward edit share its
/// id with the author's N-forward edit. Multi-op ops keep their content-hash ids.
/// The ONE identity rule of every route an edit becomes final on (audit W1G-8): `apply_command`,
/// `commit_open_transaction`, and the retained batched publication (`advance_apply_batch` stamps the
/// staged edit before its revision digest, and the open edit when a batch closes its transaction); a
/// streamed transaction tick always appends beyond index 0 of its open edit, so it never qualifies. See
/// `checkpoint_after_ingesting_a_remote_edit_stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives`
/// > for the regression this fixes.
pub(crate) fn stamp_primary_operation_identity<Mutation>(edit: &mut Edit<Mutation>) {
    if edit.forwards.len() != 1 {
        return;
    }
    if let Some(first) = edit.mutation_meta.first_mut() {
        first.mutation_id = Some(MutationId(edit.id.clone()));
    }
}

/// 🕊️ `CommitCheckpoint`'s "nothing uncommitted" rejection message — a `pub const`, not an inline
/// literal, so a caller (e.g. `VcsArtifactApp::dispatch_action` in `🔌️plugin/🦀️.rs`) can
/// recognize this SPECIFIC, benign outcome by value rather than duplicating the string. Genuinely
/// benign, not just here to silence an error: after a `CommitCheckpoint` dispatch's own `pump()`
/// absorbs a peer's checkpoint that already covers every edit this store has applied (a real,
/// expected outcome of full convergence — see
/// `checkpoint_after_ingesting_a_remote_edit_stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives`'s
/// sibling scenario), there is truly nothing left to commit — the same "requested but nothing to
/// do" shape as `VcsError::NothingToUndo`/`NothingToRedo`, just without a dedicated variant.
pub const EMPTY_CHECKPOINT_MESSAGE: &str = "cannot create an empty checkpoint";
