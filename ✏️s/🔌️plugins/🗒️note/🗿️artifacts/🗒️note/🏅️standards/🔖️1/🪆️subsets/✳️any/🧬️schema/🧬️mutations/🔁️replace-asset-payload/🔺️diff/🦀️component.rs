//! 🔺️ Diff fragment yielded by `ReplaceAssetPayload`.
use super::mutation::ReplaceAssetPayload;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_asset_upsert_diff;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceAssetPayload, _base: &NoteSnapshot) -> NoteDiff {
    note_asset_upsert_diff(&payload.key, &payload.new_asset)
}
//#endregion 🔖️Diff
