//! ↩️ Inverse for `RenameLayer` — the OLD name looked up from BASE, never a captured id.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RenameLayer, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::rename_layer(payload.layer_id.clone(), layer_base(layer).name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
