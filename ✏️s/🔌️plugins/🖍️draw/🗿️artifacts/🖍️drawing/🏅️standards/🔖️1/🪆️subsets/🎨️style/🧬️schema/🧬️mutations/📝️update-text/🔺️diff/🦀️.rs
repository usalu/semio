//! 📝️ Validated text facet delta.
pub fn diff(payload: &super::mutation::UpdateText, base: &crate::DrawingSnapshot) -> protocol::MutationOutcome<crate::DrawingDiff> {
    let text = match crate::schema::find_drawing_layer(base, &payload.layer_id) {
        Some(crate::DrawingLayerNode::Text(text)) => text,
        Some(_) => return protocol::MutationOutcome::error("mutation.target-mismatch", "The target layer is not a text layer", [payload.layer_id.to_string_owner()]),
        None => return protocol::MutationOutcome::error("mutation.target-missing", "The target text layer does not exist", [payload.layer_id.to_string_owner()]),
    };
    if !payload.size.is_finite() || payload.size <= 0.0 { return protocol::MutationOutcome::fatal("mutation.invariant", "Text size must be finite and positive", [payload.layer_id.to_string_owner()]); }
    if text.content == payload.content && text.size == payload.size && text.font_family==payload.font_family { return protocol::MutationOutcome::empty(); }
    protocol::MutationOutcome::new(crate::diff::layer_base_patch(&payload.layer_id,crate::diff::DrawingLayerPatch{text_content:(text.content!=payload.content).then(||payload.content.to_string_owner()),text_size:(text.size!=payload.size).then_some(payload.size),font_family:(text.font_family!=payload.font_family).then_some(payload.font_family),..Default::default()}))
}
