//! 🔺️ Diff fragment yielded by `ResizeBlock`.
use super::mutation::ResizeBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &ResizeBlock, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    let mut updated = block.clone();
    match &mut updated { crate::artifacts::note::NoteBlockNode::Text { width, height, .. } | crate::artifacts::note::NoteBlockNode::Image { width, height, .. } | crate::artifacts::note::NoteBlockNode::Table { width, height, .. } | crate::artifacts::note::NoteBlockNode::Math { width, height, .. } | crate::artifacts::note::NoteBlockNode::Ink { width, height, .. } | crate::artifacts::note::NoteBlockNode::Group { width, height, .. } => { *width = payload.new_width; *height = payload.new_height; } }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
