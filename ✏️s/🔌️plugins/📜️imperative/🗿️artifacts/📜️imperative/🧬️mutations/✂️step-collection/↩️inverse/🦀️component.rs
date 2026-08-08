//! ✂️step-collection `ImperativeMutation` inverse leaf.
use crate::artifacts::imperative::ImperativeDocument;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use protocol::Mutation;

pub fn inverse(base: &ImperativeDocument, mutation: &ImperativeMutation) -> Vec<ImperativeMutation> {
    <ImperativeMutation as Mutation<ImperativeDocument>>::inverse(mutation, base)
}
