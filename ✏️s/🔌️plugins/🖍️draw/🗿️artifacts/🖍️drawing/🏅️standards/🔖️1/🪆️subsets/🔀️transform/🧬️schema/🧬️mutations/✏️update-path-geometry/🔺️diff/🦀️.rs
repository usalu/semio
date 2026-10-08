//! 📐️ Typed path facet delta with finite geometry admission.
use crate::{DrawingSnapshot, DrawingLayerNode};
pub fn diff(payload: &super::mutation::UpdatePathGeometry, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
    let path = match crate::schema::find_drawing_layer(base, &payload.layer_id) {
        Some(DrawingLayerNode::Path(path)) => path,
        Some(_) => return protocol::MutationOutcome::error("mutation.target-mismatch", "The target layer is not a path", [payload.layer_id.to_string_owner()]),
        None => return protocol::MutationOutcome::error("mutation.target-missing", "The target path does not exist", [payload.layer_id.to_string_owner()]),
    };
    if !payload.segments.iter().all(crate::schema::valid_path_segment) { return protocol::MutationOutcome::fatal("mutation.invariant", "Path coordinates must be finite and radii nonnegative", [payload.layer_id.to_string_owner()]); }
    if path.segments == payload.segments { return protocol::MutationOutcome::empty(); }
    protocol::MutationOutcome::new(crate::diff::diff_set_path_geometry(&payload.layer_id, &payload.segments))
}
