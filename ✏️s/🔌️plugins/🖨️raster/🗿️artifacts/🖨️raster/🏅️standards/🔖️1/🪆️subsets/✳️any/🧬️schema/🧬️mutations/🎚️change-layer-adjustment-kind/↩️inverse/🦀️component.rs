//! ↩️ `change-layer-adjustment-kind` inverse — the old `adjustment_kind` from `base`. Not an
//! `Adjustment`, or missing target ⇒ `Vec::new()`.

use crate::artifacts::raster::engine::find_layer;
use crate::artifacts::raster::mutations::change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind;
use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeLayerAdjustmentKind, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(RasterLayerNode::Adjustment { adjustment_kind, .. }) => {
            vec![RasterMutation::ChangeLayerAdjustmentKind(ChangeLayerAdjustmentKind { layer_id: payload.layer_id.clone(), new_adjustment_kind: adjustment_kind.clone() })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
