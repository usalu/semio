//! ↩️ Restores the actual optional intrinsic dimensions and image reference through one pixel-content row.


use semio_s_artifact_raster_raster_model::find_layer;
use semio_s_artifact_raster_raster_model::RasterLayerNode;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<semio_s_artifact_raster_raster_mutations_pixels::change_layer_pixels::ChangeLayerPixels>>(payload: &crate::resize_layer::mutation::ResizeLayer, base: &RasterSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_layer(&base.layers, &payload.layer_id) {
        Some(RasterLayerNode::Pixel { image_key, width, height, .. }) => {
            vec![Op::from(semio_s_artifact_raster_raster_mutations_pixels::change_layer_pixels::ChangeLayerPixels { layer_id: payload.layer_id.clone(), expected_image_key:image_key.clone(),content:semio_s_artifact_raster_raster_model::RasterPixelContent{image_key:image_key.clone(),width:*width,height:*height},transform:None })]
        }
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
