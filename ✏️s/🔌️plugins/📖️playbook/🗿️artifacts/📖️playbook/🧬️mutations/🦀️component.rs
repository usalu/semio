//! 🧬️ playbook artifact — kernel `PlaybookMutation` facet.
pub use crate::playbook::{
    add_block_operation, add_step_operation, apply_playbook_edit_mutation, move_block_operation,
    move_step_operation, remove_block_operation, remove_step_operation, update_playbook_title_operation,
    PlaybookMutation,
};

use crate::playbook::PlaybookSpec;
use protocol::Mutation;

pub fn inverse_playbook_mutation(spec: &PlaybookSpec, mutation: &PlaybookMutation) -> Vec<PlaybookMutation> {
    <PlaybookMutation as Mutation<PlaybookSpec>>::inverse(mutation, spec)
}
