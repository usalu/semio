//! 🔺️ Sparse diff builder for `SetLayerVisible`.
use crate::artifacts::drawing::diff::{diff_set_layer_visible, DrawingDiff};
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Diff
/// 🔺️ One `visible` field patch — a real sparse `DrawingDiff`, never apply-then-capture.
pub fn diff(payload: &super::mutation::SetLayerVisible, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).visible == payload.visible {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" visible is already {}.", payload.layer_id, payload.visible));
    }
    protocol::MutationOutcome::new(diff_set_layer_visible(&payload.layer_id, payload.visible))
}
//#endregion 🔖️Diff
