//! 🔺️ `move-layer` sparse diff — writes only the layer's `transform.x`/`.y`.

use semio_s_artifact_raster_raster_diff::{diff_patch_layer, RasterDiff};
use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::{RasterLayerNode, RasterLayerPatch};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::move_layer::mutation::MoveLayer, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    let Some(layer) = find_layer(&base.layers, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" position must be finite, got ({}, {}).", payload.layer_id, payload.new_x, payload.new_y), [payload.layer_id.clone()]);
    }
    let (x, y) = match layer {
        RasterLayerNode::Pixel { transform, .. } | RasterLayerNode::Group { transform, .. } | RasterLayerNode::Adjustment { transform, .. } => (transform.x, transform.y),
    };
    if x == payload.new_x && y == payload.new_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" is already at ({}, {}).", payload.layer_id, payload.new_x, payload.new_y));
    }
    protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { transform_x: Some(payload.new_x), transform_y: Some(payload.new_y), ..Default::default() }))
}
//#endregion 🔖️Diff
