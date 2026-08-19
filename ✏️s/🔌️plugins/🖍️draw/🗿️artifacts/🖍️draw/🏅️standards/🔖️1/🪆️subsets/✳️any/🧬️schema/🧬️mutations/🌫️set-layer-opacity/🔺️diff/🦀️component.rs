//! 🔺️ Sparse diff builder for `SetLayerOpacity`.
use crate::artifacts::draw::diff::{diff_set_layer_opacity, DrawDiff};
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::SetLayerOpacity, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    let Some(layer) = find_draw_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if !payload.opacity.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Layer \"{}\" opacity must be finite, got {}.", payload.layer_id, payload.opacity), [payload.layer_id.clone()]);
    }
    if layer_base(layer).opacity == payload.opacity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" opacity is already {}.", payload.layer_id, payload.opacity));
    }
    protocol::MutationOutcome::new(diff_set_layer_opacity(&payload.layer_id, payload.opacity))
}
//#endregion 🔖️Diff
