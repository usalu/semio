//! 🔺️ Diff fragment yielded by `InsertTableColumn`. Error `target-missing` when the block is
//! absent or not a table.
use super::InsertTableColumn;
use crate::schema::diff::{NoteBlockPatch, NoteTableEdit};
use crate::{NoteBlockNode, NoteDiff, NoteSnapshot, NoteTableCell};

//#region 🔖️Diff
pub fn diff(payload: &InsertTableColumn, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let NoteBlockNode::Table { columns, rows, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a table.", payload.id), [payload.id.clone()]);
    };
    let name = payload.name.clone().unwrap_or_else(|| ((b'A' + (columns.len() as u8 % 26)) as char).to_string());
    let cells = payload.cells.clone().unwrap_or_else(|| rows.iter().map(|_| NoteTableCell { content: String::new() }).collect());
    if cells.len() != rows.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Table \"{}\" has {} row(s); the column carries {} cell(s).", payload.id, rows.len(), cells.len()), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { table: Some(vec![NoteTableEdit::InsertColumn { index: columns.len(), name, cells }]), ..Default::default() })]))
}
//#endregion 🔖️Diff
