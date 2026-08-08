use crate::artifacts::jack::JackSnapshot;
use crate::artifacts::jack::mutations::TrinityGraphMutation;

pub fn inverse(base: &JackSnapshot, mutation: &TrinityGraphMutation) -> Vec<TrinityGraphMutation> {
    <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::inverse(mutation, base)
}
