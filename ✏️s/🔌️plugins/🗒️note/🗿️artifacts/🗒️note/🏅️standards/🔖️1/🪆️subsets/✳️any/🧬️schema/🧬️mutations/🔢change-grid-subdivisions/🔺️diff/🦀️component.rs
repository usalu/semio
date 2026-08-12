//! 🔺️ Diff fragment yielded by `ChangeGridSubdivisions`.
use super::mutation::ChangeGridSubdivisions;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeGridSubdivisions, base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { grid_subdivisions: Some(payload.new_subdivisions), ..Default::default() }
}
//#endregion 🔖️Diff
