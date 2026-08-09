use crate::artifacts::dag::DagSnapshot;
use crate::artifacts::dag::mutations::DagMutation;
use protocol::Mutation;

pub fn inverse(base: &DagSnapshot, mutation: &DagMutation) -> Vec<DagMutation> {
    <DagMutation as Mutation<DagSnapshot>>::inverse(mutation, base)
}
