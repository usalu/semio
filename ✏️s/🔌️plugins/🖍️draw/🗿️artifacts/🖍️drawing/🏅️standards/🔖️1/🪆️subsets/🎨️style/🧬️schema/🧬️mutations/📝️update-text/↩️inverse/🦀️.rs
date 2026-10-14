//! ↩️ Restores the text facet from the previous document.
pub fn inverse<Op: From<crate::update_text::mutation::UpdateText>>(payload: &super::mutation::UpdateText, base: &semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match semio_s_artifact_draw_drawing_model::find_drawing_layer(base, &payload.layer_id) {
        Some(semio_s_artifact_draw_drawing_model::DrawingLayerNode::Text(text)) => vec![super::mutation::update_text(payload.layer_id.clone(), text.content.clone(), text.size,text.font_family)],
        _ => Vec::new(),
    })
}
