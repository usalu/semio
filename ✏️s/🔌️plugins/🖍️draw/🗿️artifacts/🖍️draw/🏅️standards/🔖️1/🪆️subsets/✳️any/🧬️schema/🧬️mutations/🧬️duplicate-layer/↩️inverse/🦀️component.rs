//! ↩️ Inverse for `DuplicateLayer` — `delete-layer` of the deterministic duplicate id (recomputed
//! from BASE via the same content-addressed hash `diff` used to create it). Missing source ⇒
//! `Vec::new()`.
use crate::artifacts::draw::schema::{clone_draw_layer_node, find_draw_layer, layer_id};
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DuplicateLayer, base: &DrawSnapshot) -> Vec<DrawMutation> {
    match find_draw_layer(base, &payload.layer_id) {
        Some(layer) => {
            let duplicate = clone_draw_layer_node(layer, " copy");
            vec![crate::artifacts::draw::mutations::delete_layer::mutation::delete_layer(layer_id(&duplicate).to_string())]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
