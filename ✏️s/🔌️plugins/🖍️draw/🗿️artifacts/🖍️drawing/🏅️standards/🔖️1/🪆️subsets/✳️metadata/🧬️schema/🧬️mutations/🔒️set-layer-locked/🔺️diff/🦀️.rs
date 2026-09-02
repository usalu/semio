//! 🔺️ Sparse diff builder for `SetLayerLocked`.
use crate::artifacts::drawing::diff::{diff_set_layer_locked, DrawingDiff};
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerLocked, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).locked == payload.locked {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" locked is already {}.", payload.layer_id, payload.locked));
    }
    protocol::MutationOutcome::new(diff_set_layer_locked(&payload.layer_id, payload.locked))
}
//#endregion 🔖️Diff
