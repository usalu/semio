//! 🔺️ Diff fragment yielded by `ChangeBlockVisible`. Error `target-missing` when absent, Warning
//! `no-op` when already at that visibility.
use super::mutation::ChangeBlockVisible;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockVisible, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::artifacts::note::schema::block_visible(block) == payload.new_visible {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" visible is already {}.", payload.id, payload.new_visible));
    }
    let mut updated = block.clone();
    match &mut updated { crate::artifacts::note::NoteBlockNode::Text { visible, .. } | crate::artifacts::note::NoteBlockNode::Image { visible, .. } | crate::artifacts::note::NoteBlockNode::Table { visible, .. } | crate::artifacts::note::NoteBlockNode::Math { visible, .. } | crate::artifacts::note::NoteBlockNode::Ink { visible, .. } | crate::artifacts::note::NoteBlockNode::Group { visible, .. } => *visible = payload.new_visible, }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
