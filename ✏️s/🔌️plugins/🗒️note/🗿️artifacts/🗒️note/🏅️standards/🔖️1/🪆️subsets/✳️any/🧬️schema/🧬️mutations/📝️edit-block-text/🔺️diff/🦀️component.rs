//! 🔺️ Diff fragment yielded by `EditBlockText`.
use super::mutation::EditBlockText;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &EditBlockText, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    if !matches!(block, crate::artifacts::note::NoteBlockNode::Text { .. }) { return NoteDiff::default(); }
    let mut updated = block.clone();
    if let crate::artifacts::note::NoteBlockNode::Text { paragraphs, .. } = &mut updated { *paragraphs = payload.new_paragraphs.clone(); }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
