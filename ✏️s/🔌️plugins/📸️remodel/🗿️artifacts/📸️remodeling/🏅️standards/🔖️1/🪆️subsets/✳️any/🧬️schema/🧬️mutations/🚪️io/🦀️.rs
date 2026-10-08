//! 🚪️ Apply boundary of the remodeling mutation vocabulary: the only place in this facet that turns a mutation's diff into a snapshot, through the central applier.

use super::RemodelingMutation;
use crate::RemodelingSnapshot;

/// ▶️ Applies `mutation` via its diff — kept as a free-function wrapper (matching `🎬️sequence`'s `apply_sequence_mutation`) since external callers (the editor surface) still call it by this name.
pub fn apply_remodeling_mutation(snapshot: &RemodelingSnapshot, mutation: &RemodelingMutation) -> protocol::MutationApplyResult<RemodelingSnapshot> {
    protocol::apply_diff(&protocol::Mutation::diff(mutation, snapshot).into_parts().0, snapshot)
}

/// ▶️ One diff-and-apply step, keeping the diagnostic codes the outcome raised — a rejected or
/// no-op kind is a RESULT this bridge reports, never an error it swallows.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn bridge_step(snapshot: &RemodelingSnapshot, mutation: &RemodelingMutation) -> Result<(RemodelingSnapshot, Vec<String>), String> {
    use protocol::Mutation;
    let outcome = <RemodelingMutation as Mutation<RemodelingSnapshot>>::diff(mutation, snapshot);
    let messages: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => Ok((next, messages)),
        Err(error) => Err(format!("{error:?}")),
    }
}
