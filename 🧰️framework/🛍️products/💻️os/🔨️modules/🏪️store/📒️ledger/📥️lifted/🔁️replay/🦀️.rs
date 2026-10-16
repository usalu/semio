//! 📥️ Items of `replay` that belong to `os-store-ledger` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🔁️replay/🦀️.rs`.
use semio_framework_os_spr::MutationId;
use semio_framework_os_spr::{Edit, Mutation};
use crate::CursorRevisionAccumulator;
/// 🪞️ The effective supersession of every superseded operation ([`protocol::HistoryFold::supersessions`]): the
/// replacement input every fold site reads in place of the recorded forward, which is never modified.
pub type EffectiveSupersessions = protocol::HistoryFoldIndex<MutationId, protocol::EffectiveSupersession>;

/// 🪜️ A fold's running projection: what the next operation diffs against, and how its successor displaces it.
pub trait FoldProjection<P, Mutation> {
    fn projection(&self) -> &P;
    fn advance(&mut self, next: P);
}

/// 🔏️ Content digest of one effective supersession: which transition installed it and what it replaces the input with.
pub fn supersession_digest(mutation_id: &MutationId, supersession: &protocol::EffectiveSupersession) -> [u8; 32] {
    let (kind, schema, payload): (&[u8], &[u8], &[u8]) = match &supersession.replacement {
        protocol::InputReplacement::Input { schema, payload } => (b"input", schema.as_bytes(), payload),
        protocol::InputReplacement::Withdrawn => (b"withdrawn", b"", b""),
    };
    CursorRevisionAccumulator::hash_record(b"supersession", &[mutation_id.0.as_bytes(), supersession.transition_id.as_bytes(), kind, schema, payload])
}

/// 🔐️ Digest of the supersessions in effect for `edit`'s operations, `None` when none is.
pub fn edit_supersession_digest<P, Mutation>(edit: &Edit<Mutation>, supersessions: &EffectiveSupersessions) -> Option<[u8; 32]>
where
    Mutation: self::Mutation<P>,
{
    if supersessions.is_empty() {
        return None;
    }
    let mut digest = None;
    for (index, mutation_id) in semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).iter().enumerate() {
        if let Some(supersession) = supersessions.get(mutation_id) {
            digest = Some(CursorRevisionAccumulator::hash_record(b"edit-supersessions", &[&digest.unwrap_or([0; 32]), &(index as u64).to_be_bytes(), &supersession_digest(mutation_id, supersession)]));
        }
    }
    digest
}
