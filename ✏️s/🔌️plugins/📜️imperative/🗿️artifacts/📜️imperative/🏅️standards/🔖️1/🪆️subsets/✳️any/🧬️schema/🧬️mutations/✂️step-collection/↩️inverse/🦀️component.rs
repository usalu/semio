//! ✂️step-collection `ImperativeMutation` inverse leaf.
use crate::artifacts::imperative::ImperativeSnapshot;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use protocol::Mutation;

pub fn inverse(base: &ImperativeSnapshot, mutation: &ImperativeMutation) -> Vec<ImperativeMutation> {
    <ImperativeMutation as Mutation<ImperativeSnapshot>>::inverse(mutation, base)
}
