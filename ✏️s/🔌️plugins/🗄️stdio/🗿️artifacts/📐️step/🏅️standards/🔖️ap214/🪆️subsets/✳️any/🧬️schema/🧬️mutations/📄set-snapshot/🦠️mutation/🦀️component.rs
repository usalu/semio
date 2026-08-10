use crate::artifacts::step::{StepSnapshot};
use crate::artifacts::step::schema::mutations::{StepMutation, apply_step_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut StepSnapshot, mutation: &StepMutation) {
    apply_step_mutation(projection, mutation);
}
