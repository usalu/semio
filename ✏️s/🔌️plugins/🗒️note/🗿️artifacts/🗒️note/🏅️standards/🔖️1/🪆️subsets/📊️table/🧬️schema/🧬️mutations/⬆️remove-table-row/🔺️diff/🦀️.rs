//! 🔺️ Diff fragment yielded by `RemoveTableRow`. Error `target-missing` when the block is absent
//! or not a table, Warning `no-op` when already at the 1-row floor.
use super::RemoveTableRow;
use crate::schema::diff::note_block_patch_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveTableRow, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(block) = crate::schema::find_block(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let crate::NoteBlockNode::Table { rows, .. } = block else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" is not a table.", payload.id), [payload.id.clone()]);
    };
    if rows.len() <= 1 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Table \"{}\" already has the minimum of 1 row.", payload.id));
    }
    let mut updated = block.clone();
    if let crate::NoteBlockNode::Table { rows, .. } = &mut updated {
        rows.pop();
    }
    protocol::MutationOutcome::new(note_block_patch_diff(&payload.id, &updated))
}
//#endregion 🔖️Diff
