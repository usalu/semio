//! 🔺️ Diff fragment yielded by `ChangeGridSpacing`.
use super::mutation::ChangeGridSpacing;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGridSpacing, _base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { grid_spacing: Some(payload.new_spacing), ..Default::default() }
}
//#endregion 🔖️Diff
