//! 🔺️ Sparse diff builder for `UpdateLayerTransform`.
use crate::artifacts::drawing::diff::{diff_set_layer_transform, DrawingDiff};
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateLayerTransform, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    let t = &payload.transform;
    if !t.x.is_finite() || !t.y.is_finite() || !t.scale_x.is_finite() || !t.scale_y.is_finite() || !t.rotation.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" transform must be finite.", payload.layer_id), [payload.layer_id.clone()]);
    }
    if t.scale_x <= 0.0 || t.scale_y <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" transform scale must be positive.", payload.layer_id), [payload.layer_id.clone()]);
    }
    if layer_base(layer).transform == payload.transform {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" transform is unchanged.", payload.layer_id));
    }
    protocol::MutationOutcome::new(diff_set_layer_transform(&payload.layer_id, &payload.transform))
}
//#endregion 🔖️Diff
