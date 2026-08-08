//! ✂️step-collection `ImperativeMutation` apply leaf.
use crate::artifacts::imperative::ImperativeDocument;
use crate::artifacts::imperative::mutations::ImperativeMutation;

pub fn apply(projection: &mut ImperativeDocument, mutation: &ImperativeMutation) {
    *projection = protocol::MutationDiff::apply(&<ImperativeMutation as protocol::Mutation<ImperativeDocument>>::diff(mutation, projection), projection);
}
