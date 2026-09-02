//! 🔺️ Sparse diff builder for `SetLayerBlendMode`.
use crate::artifacts::drawing::diff::{diff_set_layer_blend_mode, DrawingDiff};
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerBlendMode, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).blend_mode == payload.blend_mode {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" blend mode is already \"{}\".", payload.layer_id, payload.blend_mode));
    }
    protocol::MutationOutcome::new(diff_set_layer_blend_mode(&payload.layer_id, &payload.blend_mode))
}
//#endregion 🔖️Diff
