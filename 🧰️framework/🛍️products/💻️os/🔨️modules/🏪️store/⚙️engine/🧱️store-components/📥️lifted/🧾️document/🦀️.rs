//! 📥️ Items of `document_materialize` that belong to `os-store-engine-core` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧾️document/🦀️.rs`.
use semio_framework_os_spr::Mutation;
use std::collections::HashSet;
use semio_framework_os_store_space_schema::ArtifactEnvelope;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
/// 🕰️ Single timestamp source for `Edit.started_at`/`Checkpoint.timestamp` — re-exported so
/// callers outside this crate (e.g. the framework session command log) stamp entries in the
/// exact same format.
pub fn now_iso() -> String {
    format!("{}", now_ms())
}

/// 🩺️ Defense in depth, NOT the fix for the id-domain discontinuity itself (see {@link
/// stamp_primary_operation_identity}, which is): every id this returns is about to become a
/// `Change.edit_ids` entry, so if one isn't backed by a real `envelope.vcs.edits` entry, a much
/// louder and more specific `debug_assert!` here beats discovering it two calls later as
/// `validate_durable_history`'s cryptic "invalid edit reference". Deliberately does NOT filter the
/// id out in release builds — silently shrinking the checkpoint would drop a real edit out of
/// history without a trace, exactly the failure mode `checkpoint_after_ingesting_a_remote_edit_
/// stays_valid_once_the_sender_s_own_checkpoint_snapshot_arrives`'s doc comment warns against.
pub async fn uncommitted_edit_ids<P, Mutation, I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I) -> Vec<String>
where
    
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
