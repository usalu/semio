//! 🚪️ Apply boundary of the note mutation vocabulary: the only place in this facet that turns a mutation's diff into a snapshot, through the central applier.

use super::NoteMutation;
use crate::{NoteDiff, NoteSnapshot};

/// ▶️ Applies `mutation` via its diff — the sole apply path (no hand-written match dispatch).
pub fn apply_note_mutation(snapshot: &NoteSnapshot, mutation: &NoteMutation) -> protocol::MutationApplyResult<NoteSnapshot> {
    let (diff, _messages) = protocol::Mutation::diff(mutation, snapshot).into_parts();
    protocol::apply_diff(&diff, snapshot)
}

/// 🧮️ Applies `mutation` to `base` and hands back the whole `protocol::MutationOutcome`, the
/// diagnostics included — the shape an external conformance host needs, since a committed
/// `🎯️outcome` vector declares a status AND its diagnostic codes, and the plain apply wrapper
/// beside this one answers `Result<_, _>` and drops the messages.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn apply_note_mutation_outcome(snapshot: &mut NoteSnapshot, mutation: &NoteMutation) -> protocol::MutationOutcome<NoteDiff> {
    let outcome = <NoteMutation as protocol::Mutation<NoteSnapshot>>::diff(mutation, snapshot);
    let (next, outcome) = store::apply_outcome(snapshot, outcome);
    *snapshot = next;
    outcome
}
