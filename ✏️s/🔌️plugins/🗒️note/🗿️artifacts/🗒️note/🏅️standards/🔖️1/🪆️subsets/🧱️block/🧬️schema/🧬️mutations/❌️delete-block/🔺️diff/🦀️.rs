//! 🔺️ Diff fragment yielded by `DeleteBlock`. Error `target-missing` when the block is absent.
use super::DeleteBlock;
use crate::schema::diff::NoteBlockRow;
use crate::{NoteDiff, NoteSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some((parent_id, index)) = crate::schema::find_block_location(&base.blocks, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(NoteDiff::block_rows(vec![NoteBlockRow::Remove { id: payload.id.clone(), parent_id, index }]))
}
//#endregion 🔖️Diff
