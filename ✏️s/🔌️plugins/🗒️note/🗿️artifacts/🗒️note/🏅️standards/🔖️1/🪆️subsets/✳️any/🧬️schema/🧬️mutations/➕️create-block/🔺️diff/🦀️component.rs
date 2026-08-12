//! 🔺️ Diff fragment yielded by `CreateBlock`.
use super::mutation::CreateBlock;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_block_added_diff;

//#region 🔖️Diff
pub fn diff(payload: &CreateBlock, _base: &NoteSnapshot) -> NoteDiff {
    note_block_added_diff(payload.parent_id.clone(), payload.index, (*payload.block).clone())
}
//#endregion 🔖️Diff
