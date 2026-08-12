//! 🔺️ Diff fragment yielded by `DeleteBlocks`.
use super::mutation::DeleteBlocks;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_removed_diff;

//#region 🔖️Diff
pub fn diff(payload: &DeleteBlocks, _base: &NoteSnapshot) -> NoteDiff {
    note_block_removed_diff(payload.ids.clone())
}
//#endregion 🔖️Diff
