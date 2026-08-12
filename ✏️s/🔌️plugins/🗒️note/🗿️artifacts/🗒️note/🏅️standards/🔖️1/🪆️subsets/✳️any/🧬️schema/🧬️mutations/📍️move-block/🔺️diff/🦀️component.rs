//! 🔺️ Diff fragment yielded by `MoveBlock`.
use super::mutation::MoveBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &MoveBlock, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    let mut updated = block.clone();
    match &mut updated { crate::artifacts::note::NoteBlockNode::Text { x, y, .. } | crate::artifacts::note::NoteBlockNode::Image { x, y, .. } | crate::artifacts::note::NoteBlockNode::Table { x, y, .. } | crate::artifacts::note::NoteBlockNode::Math { x, y, .. } | crate::artifacts::note::NoteBlockNode::Ink { x, y, .. } | crate::artifacts::note::NoteBlockNode::Group { x, y, .. } => { *x = payload.new_x; *y = payload.new_y; } }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
