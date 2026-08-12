//! 🔺️ Diff fragment yielded by `ChangeBlockFontSize`.
use super::mutation::ChangeBlockFontSize;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &ChangeBlockFontSize, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    if !matches!(block, crate::artifacts::note::NoteBlockNode::Text { .. }) { return NoteDiff::default(); }
    let mut updated = block.clone();
    if let crate::artifacts::note::NoteBlockNode::Text { font_size, .. } = &mut updated { *font_size = payload.new_font_size; }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
