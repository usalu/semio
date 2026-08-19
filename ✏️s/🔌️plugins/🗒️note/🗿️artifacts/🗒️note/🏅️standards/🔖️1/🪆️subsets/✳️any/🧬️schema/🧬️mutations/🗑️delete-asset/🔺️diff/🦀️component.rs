//! 🔺️ Diff fragment yielded by `DeleteAsset`. Error `target-missing` when the key is absent.
use super::mutation::DeleteAsset;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_asset_removed_diff;

//#region 🔖️Diff
pub async fn diff(payload: &DeleteAsset, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if !base.assets.contains_key(&payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.key), [payload.key.clone()]);
    }
    protocol::MutationOutcome::new(note_asset_removed_diff(&payload.key))
}
//#endregion 🔖️Diff
