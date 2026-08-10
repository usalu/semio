//! 🩹update-block `PlaybookMutation` apply leaf.
use crate::artifacts::playbook::PlaybookSnapshot;
use crate::artifacts::playbook::mutations::PlaybookMutation;

pub fn apply(snapshot: &mut PlaybookSnapshot, mutation: &PlaybookMutation) {
    *snapshot = crate::artifacts::playbook::mutations::apply_playbook_mutation(snapshot, mutation);
}
