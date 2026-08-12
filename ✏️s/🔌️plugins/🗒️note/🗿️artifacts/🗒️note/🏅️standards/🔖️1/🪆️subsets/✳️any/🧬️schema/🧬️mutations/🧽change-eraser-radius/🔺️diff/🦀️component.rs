//! 🔺️ Diff fragment yielded by `ChangeEraserRadius`.
use super::mutation::ChangeEraserRadius;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeEraserRadius, base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { eraser_radius: Some(payload.new_radius), ..Default::default() }
}
//#endregion 🔖️Diff
