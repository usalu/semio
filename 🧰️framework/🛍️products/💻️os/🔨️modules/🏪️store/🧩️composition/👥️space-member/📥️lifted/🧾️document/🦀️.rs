//! 📥️ Items of `document_materialize` that belong to `os-store-space-member` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧾️document/🦀️.rs`.
use semio_framework_os_spr::{Mutation, OpBinary};
use semio_framework_os_vcs::VcsError;
use semio_framework_os_store_space_schema::{ArtifactEnvelope, ArtifactEnvelopeOwners};
use semio_framework_os_store_ledger::fold_envelope_history;
use semio_framework_os_store_ledger::ReplayProjection;
use semio_framework_os_store_replay::fold_effective_edit;
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
