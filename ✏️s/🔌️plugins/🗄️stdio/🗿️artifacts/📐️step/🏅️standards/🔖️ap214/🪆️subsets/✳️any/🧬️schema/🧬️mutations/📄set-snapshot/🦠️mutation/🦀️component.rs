use crate::artifacts::step::schema::mutations::{apply_step_mutation, StepMutation};
use crate::artifacts::step::{StepDiff, StepSnapshot};

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut StepSnapshot, mutation: &StepMutation) -> protocol::MutationOutcome<StepDiff> {
    apply_step_mutation(projection, mutation)
}
