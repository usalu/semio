//! 🔺️ Diff fragment yielded by `ReplaceAssetPayload`. Error `target-missing` when the key is
//! absent, Warning `no-op` when the payload is unchanged.
use super::mutation::ReplaceAssetPayload;
use crate::artifacts::note::NoteDiff;
use crate::artifacts::note::NoteSnapshot;
use crate::artifacts::note::schema::diff::note_asset_upsert_diff;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceAssetPayload, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(existing) = base.assets.get(&payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.key), [payload.key.clone()]);
    };
    if existing == &payload.new_asset {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Asset \"{}\" payload is unchanged.", payload.key));
    }
    protocol::MutationOutcome::new(note_asset_upsert_diff(&payload.key, &payload.new_asset))
}
//#endregion 🔖️Diff
