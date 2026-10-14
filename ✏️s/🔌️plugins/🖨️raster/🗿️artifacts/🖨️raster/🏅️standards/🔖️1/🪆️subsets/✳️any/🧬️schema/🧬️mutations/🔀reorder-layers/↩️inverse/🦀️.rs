//! ↩️ `reorder-layers` inverse — the layer's pre-move tree address from `base`. Missing target ⇒
//! `Vec::new()`.


use semio_s_artifact_raster_raster_model::locate_layer;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::reorder_layers::mutation::ReorderLayers>>(payload: &crate::reorder_layers::mutation::ReorderLayers, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match locate_layer(&base.layers, &payload.layer_id) {
        Some((parent_id, index)) => vec![Op::from(crate::reorder_layers::mutation::ReorderLayers { layer_id: payload.layer_id.clone(), parent_id, index })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
