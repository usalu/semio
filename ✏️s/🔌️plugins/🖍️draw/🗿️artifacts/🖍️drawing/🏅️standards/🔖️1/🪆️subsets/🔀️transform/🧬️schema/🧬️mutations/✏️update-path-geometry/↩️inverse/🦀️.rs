//! ↩️ Restores the path geometry captured from the command's base revision.
pub fn inverse(payload: &super::mutation::UpdatePathGeometry, base: &crate::DrawingSnapshot) -> Vec<crate::mutations::DrawingMutation> {
    match crate::schema::find_drawing_layer(base, &payload.layer_id) {
        Some(crate::DrawingLayerNode::Path(path)) => vec![super::mutation::update_path_geometry(payload.layer_id.clone(), path.segments.clone())],
        _ => Vec::new(),
    }
}
