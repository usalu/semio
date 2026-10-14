//! ↩️ Restores the authored image facet from the previous document.
pub fn inverse<Op: From<crate::update_image::mutation::UpdateImage>>(payload:&super::mutation::UpdateImage,base:&semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError> {
    Ok(match semio_s_artifact_draw_drawing_model::find_drawing_layer(base,&payload.layer_id) {
        Some(semio_s_artifact_draw_drawing_model::DrawingLayerNode::Image(image))=>vec![super::mutation::update_image(payload.layer_id.clone(),image.image_key.clone(),image.width,image.height)],
        _=>Vec::new(),
    })
}
