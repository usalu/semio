//! 🔺️ Diff fragment yielded by `InsertTableRow`. Error `target-missing` when the block is absent
//! or not a table.
use super::InsertTableRow;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &InsertTableRow, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !matches!(block, crate::NoteBlockNode::Table { .. }) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a table.", payload.id), [payload.id.clone()]);
    }
    let mut updated = block.clone();
    if let crate::NoteBlockNode::Table { columns, rows, .. } = &mut updated {
        let width = columns.len();
        rows.push((0..width).map(|_| crate::NoteTableCell { content: String::new() }).collect());
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, &updated))
}
//#endregion 🔖️Diff
