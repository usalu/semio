//! 🔺️ `change-layer-opacity` sparse diff — writes only the layer's `opacity` field.

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::change_layer_opacity::mutation::ChangeLayerOpacity;
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerOpacity, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    let Some(layer) = find_layer(&base.layers, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if !payload.new_opacity.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" opacity must be finite, got {}.", payload.layer_id, payload.new_opacity), [payload.layer_id.clone()]);
    }
    let opacity = match layer {
        RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. } => *opacity,
    };
    if opacity == payload.new_opacity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" opacity is already {}.", payload.layer_id, payload.new_opacity));
    }
    protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { opacity: Some(payload.new_opacity), ..Default::default() }))
}
//#endregion 🔖️Diff
