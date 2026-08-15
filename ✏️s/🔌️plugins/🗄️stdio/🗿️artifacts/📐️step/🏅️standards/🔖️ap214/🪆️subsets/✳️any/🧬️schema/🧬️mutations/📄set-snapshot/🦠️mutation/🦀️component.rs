use crate::artifacts::step::schema::mutations::{apply_step_mutation, StepMutation};
use crate::artifacts::step::StepSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut StepSnapshot, mutation: &StepMutation) {
    apply_step_mutation(projection, mutation);
}
