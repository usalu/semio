//! 🔺️ Diff fragment yielded by `DeleteBlock`. Error `target-missing` when the block is absent.
use super::DeleteBlock;
use crate::artifacts::note::schema::diff::note_block_removed_diff;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteBlock, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if crate::artifacts::note::schema::find_block(&base.blocks, &payload.id).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Block \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(note_block_removed_diff(vec![payload.id.clone()]))
}
//#endregion 🔖️Diff
