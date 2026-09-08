//! 🔺️ Diff fragment yielded by `DuplicateBlock`. Error `target-missing` on an absent source,
//! Fatal `duplicate-id` when the new block's id already exists.
use super::DuplicateBlock;
use crate::schema::diff::note_block_added_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DuplicateBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some((parent_id, index)) = crate::schema::find_block_location(&base.blocks, &payload.source_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.source_id), [payload.source_id.clone()]);
    };
    let new_id = crate::schema::block_id(&payload.block);
    if crate::schema::find_block(&base.blocks, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A block with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    protocol::MutationOutcome::new(note_block_added_diff(parent_id, Some(index + 1), (*payload.block).clone()))
}
//#endregion 🔖️Diff
