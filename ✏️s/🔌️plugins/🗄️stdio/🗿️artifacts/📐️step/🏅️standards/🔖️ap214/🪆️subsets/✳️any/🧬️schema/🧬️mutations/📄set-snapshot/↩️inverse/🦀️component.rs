use crate::artifacts::step::schema::mutations::StepMutation;
use crate::artifacts::step::StepSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &StepSnapshot, mutation: &StepMutation) -> Vec<StepMutation> {
    <StepMutation as Mutation<StepSnapshot>>::inverse(mutation, base)
}
