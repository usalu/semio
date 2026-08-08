use crate::artifacts::curate::SourcingDocument;
use crate::artifacts::curate::mutations::SourcingMutation;

pub fn inverse(base: &SourcingDocument, mutation: &SourcingMutation) -> Vec<SourcingMutation> {
    <SourcingMutation as protocol::Mutation<SourcingDocument>>::inverse(mutation, base)
}
