//! 🔺️ `change-layer-adjustment-kind` sparse diff — writes only `adjustment_kind`;
//! `RasterDiff::default()` when the addressed layer isn't an `Adjustment` (or doesn't exist).

use semio_s_artifact_raster_raster_diff::{diff_patch_layer, RasterDiff};
use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::{RasterLayerNode, RasterLayerPatch};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    match find_layer(&base.layers, &payload.layer_id) {
        None => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]),
        Some(RasterLayerNode::Adjustment { adjustment_kind, .. }) if *adjustment_kind == payload.new_adjustment_kind => {
            protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" adjustment kind is already \"{}\".", payload.layer_id, payload.new_adjustment_kind))
        }
        Some(RasterLayerNode::Adjustment { .. }) => protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { adjustment_kind: Some(payload.new_adjustment_kind.clone()), ..Default::default() })),
        Some(_) => protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" is not an adjustment layer.", payload.layer_id), [payload.layer_id.clone()]),
    }
}
//#endregion 🔖️Diff
