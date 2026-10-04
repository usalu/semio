//! ↩️ Restores the text facet from the previous document.
pub fn inverse(payload: &super::mutation::UpdateText, base: &crate::DrawingSnapshot) -> Result<Vec<crate::DrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match crate::schema::find_drawing_layer(base, &payload.layer_id) {
        Some(crate::DrawingLayerNode::Text(text)) => vec![super::mutation::update_text(payload.layer_id.clone(), text.content.clone(), text.size)],
        _ => Vec::new(),
    }

    })())
}
