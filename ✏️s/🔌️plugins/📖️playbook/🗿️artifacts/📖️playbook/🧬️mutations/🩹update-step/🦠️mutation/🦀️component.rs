//! 🩹update-step `PlaybookMutation` apply leaf.
use crate::artifacts::playbook::PlaybookSpec;
use crate::artifacts::playbook::mutations::PlaybookMutation;

pub fn apply(projection: &mut PlaybookSpec, mutation: &PlaybookMutation) {
    *projection = crate::artifacts::playbook::mutations::apply_playbook_edit_mutation(projection, mutation);
}
