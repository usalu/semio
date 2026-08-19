//! 🔺️ Sparse diff builder for `SetLayerLocked`.
use crate::artifacts::draw::diff::{diff_set_layer_locked, DrawDiff};
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::SetLayerLocked, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
    let Some(layer) = find_draw_layer(base, &payload.layer_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.layer_id), [payload.layer_id.clone()]);
    };
    if layer_base(layer).locked == payload.locked {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" locked is already {}.", payload.layer_id, payload.locked));
    }
    protocol::MutationOutcome::new(diff_set_layer_locked(&payload.layer_id, payload.locked))
}
//#endregion 🔖️Diff
