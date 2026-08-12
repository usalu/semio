//! ↩️ Inverse for `ReplaceLayerFill` — the OLD fill payload captured from BASE.
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceLayerFill, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::replace_layer_fill(payload.layer_id.clone(), layer_base(layer).attributes.fill.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
