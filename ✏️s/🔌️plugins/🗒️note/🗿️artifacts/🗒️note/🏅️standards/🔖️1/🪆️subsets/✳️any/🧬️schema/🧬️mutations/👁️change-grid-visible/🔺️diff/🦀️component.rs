//! 🔺️ Diff fragment yielded by `ChangeGridVisible`.
use super::mutation::ChangeGridVisible;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGridVisible, _base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { grid_visible: Some(payload.new_visible), ..Default::default() }
}
//#endregion 🔖️Diff
