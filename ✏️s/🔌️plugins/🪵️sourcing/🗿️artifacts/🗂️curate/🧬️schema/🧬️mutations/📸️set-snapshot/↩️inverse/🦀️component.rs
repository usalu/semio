use crate::artifacts::curate::CurateSnapshot;
use crate::artifacts::curate::mutations::SourcingMutation;

pub fn inverse(base: &CurateSnapshot, mutation: &SourcingMutation) -> Vec<SourcingMutation> {
    <SourcingMutation as protocol::Mutation<CurateSnapshot>>::inverse(mutation, base)
}
