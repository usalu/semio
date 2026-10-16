//! 📥️ Items of `document_materialize` that belong to `os-store-engine-live` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧾️document/🦀️.rs`.
use semio_framework_os_spr::MutationId;
use semio_framework_os_spr::{Edit, Mutation};
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
pub fn stamp_primary_operation_identity<Mutation>(edit: &mut Edit<Mutation>) {
    if edit.forwards.len() != 1 {
        return;
    }
    if let Some(first) = edit.mutation_meta.first_mut() {
        first.mutation_id = Some(MutationId(edit.id.clone()));
    }
}
