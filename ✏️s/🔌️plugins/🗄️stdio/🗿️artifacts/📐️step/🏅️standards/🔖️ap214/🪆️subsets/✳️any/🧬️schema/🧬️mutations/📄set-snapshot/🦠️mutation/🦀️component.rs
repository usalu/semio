use crate::artifacts::step::schema::mutations::{apply_step_mutation, StepMutation};
use crate::artifacts::step::{StepDiff, StepSnapshot};

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut StepSnapshot, mutation: &StepMutation) -> protocol::MutationOutcome<StepDiff> {
    apply_step_mutation(projection, mutation)
}
