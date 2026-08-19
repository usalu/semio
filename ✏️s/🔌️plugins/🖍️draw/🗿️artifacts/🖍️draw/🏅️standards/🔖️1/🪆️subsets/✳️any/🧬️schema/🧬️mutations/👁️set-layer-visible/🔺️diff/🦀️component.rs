//! 🔺️ Sparse diff builder for `SetLayerVisible`.
use crate::artifacts::draw::diff::{diff_set_layer_visible, DrawDiff};
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
/// 🔺️ One `visible` field patch — a real sparse `DrawDiff`, never apply-then-capture.
pub async fn diff(payload: &super::mutation::SetLayerVisible, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    let Some(layer) = find_draw_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).visible == payload.visible {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" visible is already {}.", payload.layer_id, payload.visible));
    }
    protocol::MutationOutcome::new(diff_set_layer_visible(&payload.layer_id, payload.visible))
}
//#endregion 🔖️Diff
