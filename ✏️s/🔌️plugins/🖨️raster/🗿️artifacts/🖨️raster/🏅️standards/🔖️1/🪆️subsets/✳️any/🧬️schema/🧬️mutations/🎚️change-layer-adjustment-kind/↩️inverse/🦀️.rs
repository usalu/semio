//! ↩️ `change-layer-adjustment-kind` inverse — the old `adjustment_kind` from `base`. Not an
//! `Adjustment`, or missing target ⇒ `Vec::new()`.

use crate::artifacts::raster::mutations::RasterMutation;
use crate::artifacts::raster::schema::find_layer;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeLayerAdjustmentKind, base: &RasterSnapshot) -> Vec<RasterMutation> {
    match find_layer(&base.layers, &payload.layer_id) {
        Some(RasterLayerNode::Adjustment { adjustment_kind, .. }) => {
            vec![RasterMutation::ChangeLayerAdjustmentKind(super::ChangeLayerAdjustmentKind { layer_id: payload.layer_id.clone(), new_adjustment_kind: adjustment_kind.clone() })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
