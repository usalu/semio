//! 🔺️ Sparse diff builder for `SetLayerBlendMode`.
use crate::artifacts::draw::diff::{diff_set_layer_blend_mode, DrawDiff};
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::SetLayerBlendMode, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    let Some(layer) = find_draw_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).blend_mode == payload.blend_mode {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" blend mode is already \"{}\".", payload.layer_id, payload.blend_mode));
    }
    protocol::MutationOutcome::new(diff_set_layer_blend_mode(&payload.layer_id, &payload.blend_mode))
}
//#endregion 🔖️Diff
