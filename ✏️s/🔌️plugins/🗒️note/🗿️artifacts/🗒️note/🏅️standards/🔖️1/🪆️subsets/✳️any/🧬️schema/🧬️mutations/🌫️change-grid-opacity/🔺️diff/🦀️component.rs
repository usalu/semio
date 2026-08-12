//! 🔺️ Diff fragment yielded by `ChangeGridOpacity`.
use super::mutation::ChangeGridOpacity;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGridOpacity, _base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { grid_opacity: Some(payload.new_opacity), ..Default::default() }
}
//#endregion 🔖️Diff
