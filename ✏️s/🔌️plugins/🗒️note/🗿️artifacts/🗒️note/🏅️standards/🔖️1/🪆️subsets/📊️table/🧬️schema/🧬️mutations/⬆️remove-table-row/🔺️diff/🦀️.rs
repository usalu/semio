//! 🔺️ Diff fragment yielded by `RemoveTableRow`. Error `target-missing` when the block is absent
//! or not a table, Warning `no-op` when already at the 1-row floor.
use super::RemoveTableRow;
use crate::schema::diff::{NoteBlockPatch, NoteTableEdit};
use crate::{NoteBlockNode, NoteDiff, NoteSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &RemoveTableRow, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let NoteBlockNode::Table { rows, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a table.", payload.id), [payload.id.clone()]);
    };
    if rows.len() <= 1 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Table \"{}\" already has the minimum of 1 row.", payload.id));
    }
    protocol::MutationOutcome::new(NoteDiff::block_patches([(payload.id.clone(), NoteBlockPatch { table: Some(vec![NoteTableEdit::RemoveRow { index: rows.len() - 1 }]), ..Default::default() })]))
}
//#endregion 🔖️Diff
