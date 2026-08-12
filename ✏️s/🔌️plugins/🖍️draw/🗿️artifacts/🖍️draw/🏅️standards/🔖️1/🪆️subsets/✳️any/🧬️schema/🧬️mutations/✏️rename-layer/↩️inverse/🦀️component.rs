//! ↩️ Inverse for `RenameLayer` — the OLD name looked up from BASE, never a captured id.
use crate::artifacts::draw::schema::{find_draw_layer, layer_base};
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RenameLayer, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::rename_layer(payload.layer_id.clone(), layer_base(layer).name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
