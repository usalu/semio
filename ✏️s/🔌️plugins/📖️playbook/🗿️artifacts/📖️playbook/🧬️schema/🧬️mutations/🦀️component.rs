//! 🧬️ playbook artifact — kernel `PlaybookMutation` facet with plugin snapshot typing.
pub use crate::playbook::{
    add_block_operation, add_step_operation, apply_playbook_edit_mutation, move_block_operation,
    move_step_operation, remove_block_operation, remove_step_operation, update_playbook_title_operation,
    PlaybookMutation,
};

use crate::artifacts::playbook::schema::diff::text::playbook_diff_from_mutation;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};
use protocol::{Mutation, MutationDiff};

/// 🔄 Applies a kernel mutation onto a plugin snapshot.
pub fn apply_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> PlaybookSnapshot {
  PlaybookDiff::apply(&playbook_diff_from_mutation(mutation, snapshot), snapshot)
}

pub fn inverse_playbook_mutation(snapshot: &PlaybookSnapshot, mutation: &PlaybookMutation) -> Vec<PlaybookMutation> {
    let kernel = snapshot.as_kernel();
    <PlaybookMutation as Mutation<crate::playbook::PlaybookSpec>>::inverse(mutation, &kernel)
}

impl Mutation<PlaybookSnapshot> for PlaybookMutation {
    type Diff = crate::artifacts::playbook::PlaybookDiff;

    fn diff(&self, base: &PlaybookSnapshot) -> Self::Diff {
        playbook_diff_from_mutation(self, base)
    }

    fn inverse(&self, base: &PlaybookSnapshot) -> Vec<Self> {
        inverse_playbook_mutation(base, self)
    }
}

