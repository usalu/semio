//! 🔺️ Diff fragment yielded by `ChangeBlockVisible`. Error `target-missing` when absent, Warning
//! `no-op` when already at that visibility.
use super::ChangeBlockVisible;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockVisible, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if crate::schema::block_visible(block) == payload.new_visible {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Block \"{}\" visible is already {}.", payload.id, payload.new_visible));
    }
    let mut updated = block.clone();
    match &mut updated {
        crate::NoteBlockNode::Text { visible, .. }
        | crate::NoteBlockNode::Image { visible, .. }
        | crate::NoteBlockNode::Table { visible, .. }
        | crate::NoteBlockNode::Math { visible, .. }
        | crate::NoteBlockNode::Ink { visible, .. }
        | crate::NoteBlockNode::Group { visible, .. } => *visible = payload.new_visible,
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
