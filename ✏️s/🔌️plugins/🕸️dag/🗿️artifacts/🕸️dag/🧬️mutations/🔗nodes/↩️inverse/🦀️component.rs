use crate::artifacts::dag::DagDocument;
use crate::artifacts::dag::mutations::DagMutation;
use protocol::Mutation;

pub fn inverse(base: &DagDocument, mutation: &DagMutation) -> Vec<DagMutation> {
    <DagMutation as Mutation<DagDocument>>::inverse(mutation, base)
}
