//! ↩️ Inverse for `UpdateLayerTransform` — the OLD transform captured from BASE.
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::UpdateLayerTransform, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::update_layer_transform(payload.layer_id.clone(), layer_base(layer).transform.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
