//! ➕add-step `PlaybookMutation` inverse leaf.
use crate::artifacts::playbook::PlaybookSnapshot;
use crate::artifacts::playbook::mutations::PlaybookMutation;
use protocol::Mutation;

pub fn inverse(base: &PlaybookSnapshot, mutation: &PlaybookMutation) -> Vec<PlaybookMutation> {
    <PlaybookMutation as Mutation<PlaybookSnapshot>>::inverse(mutation, base)
}
