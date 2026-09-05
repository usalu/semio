//! ↩️ Inverse for `DuplicateLayer` — `delete-layer` of the deterministic duplicate id (recomputed
//! from BASE via the same content-addressed hash `diff` used to create it). Missing source ⇒
//! `Vec::new()`.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::{clone_drawing_layer_node, find_drawing_layer, layer_id};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DuplicateLayer, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => {
            let duplicate = clone_drawing_layer_node(layer, " copy");
            vec![crate::artifacts::drawing::mutations::delete_layer::mutation::delete_layer(layer_id(&duplicate).to_string())]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
