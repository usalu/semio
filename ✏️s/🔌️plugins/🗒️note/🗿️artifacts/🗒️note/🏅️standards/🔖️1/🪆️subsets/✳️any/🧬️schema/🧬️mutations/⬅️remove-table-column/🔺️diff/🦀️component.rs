//! 🔺️ Diff fragment yielded by `RemoveTableColumn`.
use super::mutation::RemoveTableColumn;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &RemoveTableColumn, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::engine::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    if !matches!(block, crate::artifacts::note::NoteBlockNode::Table { .. }) { return NoteDiff::default(); }
    let mut updated = block.clone();
    let mut changed = false;
    if let crate::artifacts::note::NoteBlockNode::Table { columns, rows, .. } = &mut updated { if columns.len() > 1 { columns.pop(); for row in rows.iter_mut() { row.pop(); } changed = true; } }
    if !changed { return NoteDiff::default(); }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
