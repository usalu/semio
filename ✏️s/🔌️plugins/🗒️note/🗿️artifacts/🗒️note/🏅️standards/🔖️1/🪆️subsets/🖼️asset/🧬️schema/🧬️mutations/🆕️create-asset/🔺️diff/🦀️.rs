//! 🔺️ Diff fragment yielded by `CreateAsset`. Fatal `duplicate-id` on an existing key.
use super::CreateAsset;
use crate::schema::diff::note_asset_upsert_diff;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &CreateAsset, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    if base.assets.contains_key(&payload.key) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An asset with key \"{}\" already exists.", payload.key), [payload.key.clone()]);
    }
    protocol::MutationOutcome::new(note_asset_upsert_diff(&payload.key, &payload.asset))
}
//#endregion 🔖️Diff
