//! ↩️ Inverse for `ReplaceLayerFill` — the OLD fill payload captured from BASE.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceLayerFill, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::replace_layer_fill(payload.layer_id.clone(), layer_base(layer).attributes.fill.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
