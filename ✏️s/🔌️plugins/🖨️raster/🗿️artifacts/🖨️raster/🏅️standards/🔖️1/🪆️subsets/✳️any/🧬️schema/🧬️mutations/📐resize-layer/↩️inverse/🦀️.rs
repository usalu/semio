//! ↩️ `resize-layer` inverse — the old `width`/`height` from `base` (defaulting like
//! `apply_layer_patch` does when unset). Not a `Pixel`, or missing target ⇒ `Vec::new()`.


use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::RasterLayerNode;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::resize_layer::mutation::ResizeLayer>>(payload: &crate::resize_layer::mutation::ResizeLayer, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_layer(&base.layers, &payload.layer_id) {
        Some(RasterLayerNode::Pixel { width, height, .. }) => {
            vec![Op::from(crate::resize_layer::mutation::ResizeLayer { layer_id: payload.layer_id.clone(), new_width: width.unwrap_or(512), new_height: height.unwrap_or(512) })]
        }
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
