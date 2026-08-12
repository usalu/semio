//! 🔺️ Diff fragment yielded by `ChangeBlockInkWidth`.
use super::mutation::ChangeBlockInkWidth;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockInkWidth, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    if !matches!(block, crate::artifacts::note::NoteBlockNode::Ink { .. }) { return NoteDiff::default(); }
    let mut updated = block.clone();
    if let crate::artifacts::note::NoteBlockNode::Ink { stroke_width, .. } = &mut updated { *stroke_width = payload.new_stroke_width; }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
