//! ➕add-block `PlaybookMutation` inverse leaf.
use crate::artifacts::playbook::PlaybookSpec;
use crate::artifacts::playbook::mutations::PlaybookMutation;
use protocol::Mutation;

pub fn inverse(base: &PlaybookSpec, mutation: &PlaybookMutation) -> Vec<PlaybookMutation> {
    <PlaybookMutation as Mutation<PlaybookSpec>>::inverse(mutation, base)
}
