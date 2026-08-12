//! 🔺️ Diff fragment yielded by `DeleteBlock`.
use super::mutation::DeleteBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_removed_diff;

//#region 🔖️Diff
pub fn diff(payload: &DeleteBlock, _base: &NoteSnapshot) -> NoteDiff {
    note_block_removed_diff(vec![payload.id.clone()])
}
//#endregion 🔖️Diff
