//! ↩️ Restores the path geometry captured from the command's base revision.
pub fn inverse<Op: From<crate::update_path_geometry::mutation::UpdatePathGeometry>>(payload: &super::mutation::UpdatePathGeometry, base: &semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match semio_s_artifact_draw_drawing_model::find_drawing_layer(base, &payload.layer_id) {
        Some(semio_s_artifact_draw_drawing_model::DrawingLayerNode::Path(path)) => vec![super::mutation::update_path_geometry(payload.layer_id.clone(), path.segments.clone())],
        _ => Vec::new(),
    })
}
