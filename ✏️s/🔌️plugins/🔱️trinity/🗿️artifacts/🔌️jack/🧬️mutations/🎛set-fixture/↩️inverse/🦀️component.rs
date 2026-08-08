use crate::artifacts::jack::TrinityGraphDocument;
use crate::artifacts::jack::mutations::TrinityGraphMutation;

pub fn inverse(base: &TrinityGraphDocument, mutation: &TrinityGraphMutation) -> Vec<TrinityGraphMutation> {
    <TrinityGraphMutation as protocol::Mutation<TrinityGraphDocument>>::inverse(mutation, base)
}
