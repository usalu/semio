//! 🔺️ Diff fragment yielded by `DeleteAsset`. Error `target-missing` when the key is absent.
use super::DeleteAsset;
use crate::schema::diff::NoteAssetRow;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteAsset, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if !base.assets.contains_key(&payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.key), [payload.key.clone()]);
    }
    protocol::MutationOutcome::new(NoteDiff::asset_rows(vec![NoteAssetRow::Remove { key: payload.key.clone() }]))
}
//#endregion 🔖️Diff
