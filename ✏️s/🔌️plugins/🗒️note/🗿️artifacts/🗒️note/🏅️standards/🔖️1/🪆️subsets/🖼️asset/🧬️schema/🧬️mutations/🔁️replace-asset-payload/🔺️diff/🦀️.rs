//! 🔺️ Diff fragment yielded by `ReplaceAssetPayload`. Error `target-missing` when the key is
//! absent, Warning `no-op` when the payload is unchanged.
use super::ReplaceAssetPayload;
use crate::schema::diff::NoteAssetRow;
use crate::NoteDiff;
use crate::NoteSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceAssetPayload, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
    let Some(existing) = base.assets.get(&payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Asset \"{}\" does not exist.", payload.key), [payload.key.clone()]);
    };
    if existing == &payload.new_asset {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Asset \"{}\" payload is unchanged.", payload.key));
    }
    protocol::MutationOutcome::new(NoteDiff::asset_rows(vec![NoteAssetRow::Replace { key: payload.key.clone(), asset: payload.new_asset.clone() }]))
}
//#endregion 🔖️Diff
