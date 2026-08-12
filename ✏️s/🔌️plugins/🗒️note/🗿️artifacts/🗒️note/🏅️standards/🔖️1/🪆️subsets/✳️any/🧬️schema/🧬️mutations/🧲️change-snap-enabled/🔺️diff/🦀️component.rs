//! 🔺️ Diff fragment yielded by `ChangeSnapEnabled`.
use super::mutation::ChangeSnapEnabled;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSnapEnabled, _base: &NoteSnapshot) -> NoteDiff {
    NoteDiff { snap_enabled: Some(payload.new_enabled), ..Default::default() }
}
//#endregion 🔖️Diff
