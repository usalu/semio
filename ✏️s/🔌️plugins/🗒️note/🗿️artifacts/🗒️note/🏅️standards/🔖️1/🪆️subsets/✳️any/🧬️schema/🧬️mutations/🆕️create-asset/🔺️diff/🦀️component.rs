//! 🔺️ Diff fragment yielded by `CreateAsset`.
use super::mutation::CreateAsset;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_asset_upsert_diff;

//#region 🔖️Diff
pub fn diff(payload: &CreateAsset, _base: &NoteSnapshot) -> NoteDiff {
    note_asset_upsert_diff(&payload.key, &payload.asset)
}
//#endregion 🔖️Diff
