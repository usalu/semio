//! ↩️ Inverse for `SetLayerOpacity` — reconstructed from BASE state.
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::SetLayerOpacity, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_opacity(payload.layer_id.clone(), layer_base(layer).opacity)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
