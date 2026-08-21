//! ↩️ Inverse for `SetLayerLocked` — reconstructed from BASE state.
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::SetLayerLocked, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_locked(payload.layer_id.clone(), layer_base(layer).locked)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
