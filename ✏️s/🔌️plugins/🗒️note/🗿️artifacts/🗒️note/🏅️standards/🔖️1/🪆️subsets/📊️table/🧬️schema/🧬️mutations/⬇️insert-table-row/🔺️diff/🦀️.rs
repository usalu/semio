//! 🔺️ Diff fragment yielded by `InsertTableRow`. Error `target-missing` when the block is absent
//! or not a table.
use super::InsertTableRow;
use crate::schema::diff::{NoteBlockPatch, NoteTableEdit};
use crate::{NoteBlockNode, NoteDiff, NoteSnapshot, NoteTableCell};

//#region 🔖️Diff
pub fn diff(payload: &InsertTableRow, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let NoteBlockNode::Table { columns, rows, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a table.", payload.id), [payload.id.clone()]);
    };
    let cells = payload.cells.clone().unwrap_or_else(|| (0..columns.len()).map(|_| NoteTableCell { content: String::new() }).collect());
    if cells.len() != columns.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Table \"{}\" has {} column(s); the row carries {} cell(s).", payload.id, columns.len(), cells.len()), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { table: Some(vec![NoteTableEdit::InsertRow { index: rows.len(), cells }]), ..Default::default() })]))
}
//#endregion 🔖️Diff
