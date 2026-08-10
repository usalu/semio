use crate::artifacts::step::{StepSnapshot};
use crate::artifacts::step::schema::mutations::StepMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &StepSnapshot, mutation: &StepMutation) -> Vec<StepMutation> {
    <StepMutation as Mutation<StepSnapshot>>::inverse(mutation, base)
}
