use crate::schema::mutations::StepMutation;
use crate::StepSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &StepSnapshot, mutation: &StepMutation) -> Vec<StepMutation> {
    <StepMutation as Mutation<StepSnapshot>>::inverse(mutation, base)
}
