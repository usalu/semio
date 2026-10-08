//! 🔺️ Sparse diff builder for `UpdateLayerTransform`.
use crate::diff::{diff_set_layer_transform, DrawingDiff};
use crate::schema::{find_drawing_layer, layer_base};
use crate::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdateLayerTransform, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.to_string_owner()]);
    };
    let t = &payload.transform;
    if !t.x.is_finite() || !t.y.is_finite() || !t.scale_x.is_finite() || !t.scale_y.is_finite() || !t.rotation.is_finite() || !t.shear.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" transform must be finite.", payload.layer_id), [payload.layer_id.to_string_owner()]);
    }
    if layer_base(layer).transform == payload.transform {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" transform is unchanged.", payload.layer_id));
    }
    protocol::MutationOutcome::new(diff_set_layer_transform(&payload.layer_id, &payload.transform))
}
//#endregion 🔖️Diff
