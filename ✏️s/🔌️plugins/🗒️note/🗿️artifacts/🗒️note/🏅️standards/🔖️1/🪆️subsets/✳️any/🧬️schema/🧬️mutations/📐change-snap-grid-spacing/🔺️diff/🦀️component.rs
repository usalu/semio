//! 🔺️ Diff fragment yielded by `ChangeSnapGridSpacing`.
use super::mutation::ChangeSnapGridSpacing;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSnapGridSpacing, base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { snap_grid_spacing: Some(payload.new_spacing), ..Default::default() }
}
//#endregion 🔖️Diff
