//! 🔺️ Diff fragment yielded by `InsertTableColumn`.
use super::mutation::InsertTableColumn;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_patch_diff;

//#region 🔖️Diff
pub fn diff(payload: &InsertTableColumn, base: &NoteSnapshot) -> NoteDiff {
    let Some(block) = crate::artifacts::note::schema::find_block(&base.blocks, &payload.id) else { return NoteDiff::default() };
    if !matches!(block, crate::artifacts::note::NoteBlockNode::Table { .. }) { return NoteDiff::default(); }
    let mut updated = block.clone();
    if let crate::artifacts::note::NoteBlockNode::Table { columns, rows, .. } = &mut updated {
        let next_letter = (b'A' + (columns.len() as u8 % 26)) as char;
        columns.push(next_letter.to_string());
        for row in rows.iter_mut() { row.push(crate::artifacts::note::NoteTableCell { content: String::new() }); }
    }
    note_block_patch_diff(&payload.id, updated)
}
//#endregion 🔖️Diff
