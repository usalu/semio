//! 🔺️ `change-layer-blend-mode` sparse diff — writes only the layer's `blend_mode` field.

use semio_s_artifact_raster_raster_diff::{diff_patch_layer, RasterDiff};
use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::{RasterLayerNode, RasterLayerPatch};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::change_layer_blend_mode::mutation::ChangeLayerBlendMode, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    let Some(layer) = find_layer(&base.layers, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    let blend_mode = match layer {
        RasterLayerNode::Pixel { blend_mode, .. } | RasterLayerNode::Group { blend_mode, .. } | RasterLayerNode::Adjustment { blend_mode, .. } => blend_mode,
    };
    if blend_mode == &payload.new_blend_mode {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" blend mode is already \"{}\".", payload.layer_id, payload.new_blend_mode));
    }
    protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { blend_mode: Some(payload.new_blend_mode.clone()), ..Default::default() }))
}
//#endregion 🔖️Diff
