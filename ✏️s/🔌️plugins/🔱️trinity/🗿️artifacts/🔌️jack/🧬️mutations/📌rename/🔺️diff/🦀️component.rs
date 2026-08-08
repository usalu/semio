use crate::artifacts::jack::diff::TrinityGraphDiff;
use crate::artifacts::jack::TrinityGraphDocument;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &TrinityGraphMutation, base: &TrinityGraphDocument) -> TrinityGraphDiff {
    <TrinityGraphMutation as protocol::Mutation<TrinityGraphDocument>>::diff(mutation, base)
}
