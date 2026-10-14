//! 🔺️ Sparse diff builder for `SetLayerOpacity`.
use semio_s_artifact_draw_drawing_diff::{diff_set_layer_opacity, DrawingDiff};
use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::SetLayerOpacity, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
    let Some(layer) = find_drawing_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.to_string_owner()]);
    };
    if !payload.opacity.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" opacity must be finite, got {}.", payload.layer_id, payload.opacity), [payload.layer_id.to_string_owner()]);
    }
    if layer_base(layer).opacity == payload.opacity {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Layer \"{}\" opacity is already {}.", payload.layer_id, payload.opacity));
    }
    protocol::MutationOutcome::new(diff_set_layer_opacity(&payload.layer_id, payload.opacity))
}
//#endregion 🔖️Diff
