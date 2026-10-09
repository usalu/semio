//! ↩️ Restores the authored image facet from the previous document.
pub fn inverse(payload:&super::mutation::UpdateImage,base:&crate::DrawingSnapshot)->Result<Vec<crate::DrawingMutation>,semio_framework_value::ValueError> {
    Ok(match crate::schema::find_drawing_layer(base,&payload.layer_id) {
        Some(crate::DrawingLayerNode::Image(image))=>vec![super::mutation::update_image(payload.layer_id.clone(),image.image_key.clone(),image.width,image.height)],
        _=>Vec::new(),
    })
}
