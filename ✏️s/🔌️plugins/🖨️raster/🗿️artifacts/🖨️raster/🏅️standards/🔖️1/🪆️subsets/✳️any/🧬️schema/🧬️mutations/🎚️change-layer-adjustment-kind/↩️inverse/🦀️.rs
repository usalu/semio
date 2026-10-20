//! ↩️ `change-layer-adjustment-kind` inverse — the old `adjustment_kind` from `base`. Not an
//! `Adjustment`, or missing target ⇒ `Vec::new()`.


use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::RasterLayerNode;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind>>(payload: &crate::change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_layer(&base.layers, &payload.layer_id) {
        Some(RasterLayerNode::Adjustment { adjustment_kind, .. }) => {
            vec![Op::from(crate::change_layer_adjustment_kind::mutation::ChangeLayerAdjustmentKind { layer_id: payload.layer_id.clone(), new_adjustment_kind: adjustment_kind.clone() })]
        }
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
