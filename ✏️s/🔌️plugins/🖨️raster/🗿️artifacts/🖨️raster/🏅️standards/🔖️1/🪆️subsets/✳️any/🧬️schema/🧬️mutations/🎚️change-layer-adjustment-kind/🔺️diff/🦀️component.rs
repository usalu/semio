//! 🔺️ `change-layer-adjustment-kind` sparse diff — writes only `adjustment_kind`;
//! `RasterDiff::default()` when the addressed layer isn't an `Adjustment` (or doesn't exist).

use crate::artifacts::raster::diff::{diff_patch_layer, RasterDiff};
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::mutations::change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind;
use crate::artifacts::raster::{RasterLayerNode, RasterLayerPatch, RasterSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeLayerAdjustmentKind, base: &RasterSnapshot) -> RasterDiff {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(RasterLayerNode::Adjustment { .. }) => diff_patch_layer(&payload.layer_id, RasterLayerPatch { adjustment_kind: Some(payload.new_adjustment_kind.clone()), ..Default::default() }),
        _ => RasterDiff::default(),
    }
}
//#endregion 🔖️Diff
