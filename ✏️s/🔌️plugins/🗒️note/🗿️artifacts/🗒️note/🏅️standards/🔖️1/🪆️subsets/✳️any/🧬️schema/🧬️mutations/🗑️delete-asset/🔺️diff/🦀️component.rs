//! 🔺️ Diff fragment yielded by `DeleteAsset`.
use super::mutation::DeleteAsset;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_asset_removed_diff;

//#region 🔖️Diff
pub fn diff(payload: &DeleteAsset, base: &NoteSnapshot) -> NoteDiff {
    note_asset_removed_diff(&payload.key)
}
//#endregion 🔖️Diff
