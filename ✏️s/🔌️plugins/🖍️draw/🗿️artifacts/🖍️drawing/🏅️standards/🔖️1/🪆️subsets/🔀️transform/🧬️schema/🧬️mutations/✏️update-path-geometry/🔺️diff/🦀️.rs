//! 📐️ Typed path facet delta with finite geometry admission.
use crate::{DrawingSnapshot, DrawingLayerNode};
pub fn diff(payload: &super::mutation::UpdatePathGeometry, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
    let Some(DrawingLayerNode::Path(path)) = crate::schema::find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.path-missing", "The target is not an existing path", [payload.layer_id.clone()]);
    };
    if !payload.segments.iter().all(crate::schema::valid_path_segment) { return protocol::MutationOutcome::error("mutation.invalid-geometry", "Path coordinates must be finite and radii nonnegative", [payload.layer_id.clone()]); }
    if path.segments == payload.segments { return protocol::MutationOutcome::empty(); }
    protocol::MutationOutcome::new(crate::diff::diff_set_path_geometry(&payload.layer_id, &payload.segments))
}
