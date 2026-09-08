//! ↩️ Inverse for `ReplaceLayerStroke` — the OLD stroke payload captured from BASE.
use crate::mutations::DrawingMutation;
use crate::schema::{find_drawing_layer, layer_base};
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ReplaceLayerStroke, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::replace_layer_stroke(payload.layer_id.clone(), layer_base(layer).attributes.stroke.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
