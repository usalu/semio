//! 📝️ Validated text facet delta.
pub fn diff(payload: &super::mutation::UpdateText, base: &crate::DrawingSnapshot) -> protocol::MutationOutcome<crate::DrawingDiff> {
    let Some(crate::DrawingLayerNode::Text(text)) = crate::schema::find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.text-missing", "The target is not an existing text layer", [payload.layer_id.clone()]);
    };
    if !payload.size.is_finite() || payload.size <= 0.0 { return protocol::MutationOutcome::error("mutation.invalid-text-size", "Text size must be finite and positive", [payload.layer_id.clone()]); }
    if text.content == payload.content && text.size == payload.size { return protocol::MutationOutcome::empty(); }
    protocol::MutationOutcome::new(crate::diff::diff_set_text(&payload.layer_id, &payload.content, payload.size))
}
