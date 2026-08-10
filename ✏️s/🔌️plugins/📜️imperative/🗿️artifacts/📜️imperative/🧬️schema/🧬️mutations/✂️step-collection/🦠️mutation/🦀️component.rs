//! ✂️step-collection `ImperativeMutation` apply leaf.
use crate::artifacts::imperative::ImperativeSnapshot;
use crate::artifacts::imperative::mutations::ImperativeMutation;

pub fn apply(snapshot: &mut ImperativeSnapshot, mutation: &ImperativeMutation) {
    *snapshot = protocol::MutationDiff::apply(&<ImperativeMutation as protocol::Mutation<ImperativeSnapshot>>::diff(mutation, snapshot), snapshot);
}
