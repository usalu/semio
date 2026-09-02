//! ↩️ Inverse for `ReplaceLayerStroke` — the OLD stroke payload captured from BASE.
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceLayerStroke, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::replace_layer_stroke(payload.layer_id.clone(), layer_base(layer).attributes.stroke.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
