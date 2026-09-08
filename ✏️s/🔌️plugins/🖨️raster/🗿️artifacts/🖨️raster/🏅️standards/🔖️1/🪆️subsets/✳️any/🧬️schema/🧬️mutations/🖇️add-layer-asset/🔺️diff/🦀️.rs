//! 🔺️ `add-layer-asset` sparse diff — a single `assets.entries` insertion.

use crate::diff::{diff_add_asset, RasterDiff};
use crate::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddLayerAsset, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    if base.assets.contains_key(&payload.asset_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Asset \"{}\" is already attached.", payload.asset_id));
    }
    protocol::MutationOutcome::new(diff_add_asset(&payload.asset_id, payload.asset.clone()))
}
//#endregion 🔖️Diff
