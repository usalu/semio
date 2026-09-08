//! 🔺️ Diff fragment yielded by `InsertTableColumn`. Error `target-missing` when the block is
//! absent or not a table.
use super::InsertTableColumn;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &InsertTableColumn, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !matches!(block, crate::NoteBlockNode::Table { .. }) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a table.", payload.id), [payload.id.clone()]);
    }
    let mut updated = block.clone();
    if let crate::NoteBlockNode::Table { columns, rows, .. } = &mut updated {
        let next_letter = (b'A' + (columns.len() as u8 % 26)) as char;
        columns.push(next_letter.to_string());
        for row in rows.iter_mut() {
            row.push(crate::NoteTableCell { content: String::new() });
        }
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, updated))
}
//#endregion 🔖️Diff
