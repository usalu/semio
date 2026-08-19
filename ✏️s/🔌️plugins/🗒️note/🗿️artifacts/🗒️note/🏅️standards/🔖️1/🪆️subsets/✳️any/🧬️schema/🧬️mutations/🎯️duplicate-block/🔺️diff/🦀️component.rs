//! 🔺️ Diff fragment yielded by `DuplicateBlock`. Error `target-missing` on an absent source,
//! Fatal `duplicate-id` when the new block's id already exists.
use super::mutation::DuplicateBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_added_diff;

//#region 🔖️Diff
pub async fn diff(payload: &DuplicateBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some((parent_id, index)) = crate::artifacts::note::schema::find_block_location(&base.blocks, &payload.source_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.source_id), [payload.source_id.clone()]);
    };
    let new_id = crate::artifacts::note::schema::block_id(&payload.block);
    if crate::artifacts::note::schema::find_block(&base.blocks, new_id).is_some() {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A block with id \"{}\" already exists.", new_id), [new_id.to_string()]);
    }
    protocol::MutationOutcome::new(note_block_added_diff(parent_id, Some(index + 1), (*payload.block).clone()))
}
//#endregion 🔖️Diff
