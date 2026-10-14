//! ↩️ `rename-layer` inverse — the old name from `base`; `rename` is its own inverse partner (per
//! `📓️taxonomy.md`). Missing target ⇒ `Vec::new()`.


use semio_s_artifact_raster_raster_model::{find_layer, layer_name};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::rename_layer::mutation::RenameLayer>>(payload: &crate::rename_layer::mutation::RenameLayer, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_layer(&base.layers, &payload.layer_id) {
        Some(layer) => vec![Op::from(crate::rename_layer::mutation::RenameLayer { layer_id: payload.layer_id.clone(), new_name: layer_name(layer).to_string() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
