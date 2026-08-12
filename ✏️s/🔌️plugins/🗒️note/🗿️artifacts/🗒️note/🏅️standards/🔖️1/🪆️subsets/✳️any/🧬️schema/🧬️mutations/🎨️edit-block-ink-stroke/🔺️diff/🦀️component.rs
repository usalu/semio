//! 🔺️ Diff fragment yielded by `EditBlockInkStroke`.
use super::mutation::EditBlockInkStroke;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &EditBlockInkStroke, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    if !matches!(block, crate::artifacts::note::NoteBlockNode::Ink { .. }) { return NoteDiff::default(); }
    let mut updated = block.clone();
    if let crate::artifacts::note::NoteBlockNode::Ink { points, x, y, width, height, .. } = &mut updated { *points = payload.new_points.clone(); *x = payload.new_x; *y = payload.new_y; *width = payload.new_width; *height = payload.new_height; }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
