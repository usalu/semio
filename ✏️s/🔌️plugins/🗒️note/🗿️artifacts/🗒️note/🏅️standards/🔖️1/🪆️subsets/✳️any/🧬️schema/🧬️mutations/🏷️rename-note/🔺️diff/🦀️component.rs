//! 🔺️ Diff fragment yielded by `RenameNote`.
use super::mutation::RenameNote;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RenameNote, base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { title: Some(payload.new_title.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
