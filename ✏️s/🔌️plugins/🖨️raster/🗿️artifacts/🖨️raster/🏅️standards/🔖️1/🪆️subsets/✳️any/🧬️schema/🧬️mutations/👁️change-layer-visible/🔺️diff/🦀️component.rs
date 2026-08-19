//! 🔺️ `change-layer-visible` sparse diff — writes only the layer's `visible` field.

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::mutations::change_layer_visible::mutation::ChangeLayerVisible;
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeLayerVisible, base: &RasterSnapshot) -> protocol::MutationOutcome<RasterDiff> {
    let Some(layer) = find_layer(&base.layers, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    let visible = match layer {
        RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. } => *visible,
    };
    if visible == payload.new_visible {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" visible is already {}.", payload.layer_id, payload.new_visible));
    }
    protocol::MutationOutcome::new(diff_patch_layer(&payload.layer_id, RasterLayerPatch { visible: Some(payload.new_visible), ..Default::default() }))
}
//#endregion 🔖️Diff
