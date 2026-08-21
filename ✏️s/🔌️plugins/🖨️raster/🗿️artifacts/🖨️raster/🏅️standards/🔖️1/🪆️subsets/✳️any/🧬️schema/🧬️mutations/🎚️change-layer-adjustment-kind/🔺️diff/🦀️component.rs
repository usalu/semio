//! 🔺️ `change-layer-adjustment-kind` sparse diff — writes only `adjustment_kind`;
//! `RasterDiff::default()` when the addressed layer isn't an `Adjustment` (or doesn't exist).

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind;
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeLayerAdjustmentKind, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    match find_layer(&base.layers, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]),
        Some(RasterLayerNode::Adjustment { adjustment_kind, .. }) if *adjustment_kind == payload.new_adjustment_kind => {
            protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" adjustment kind is already \"{}\".", payload.layer_id, payload.new_adjustment_kind))
        }
        Some(RasterLayerNode::Adjustment { .. }) => protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { adjustment_kind: Some(payload.new_adjustment_kind.clone()), ..Default::default() })),
        Some(_) => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" is not an adjustment layer.", payload.layer_id), [payload.layer_id.clone()]),
    }
}
//#endregion 🔖️Diff
