//! 🔺️ Diff fragment yielded by `ChangePencilWidth`.
use super::mutation::ChangePencilWidth;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePencilWidth, base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { pencil_width: Some(payload.new_width), ..Default::default() }
}
//#endregion 🔖️Diff
