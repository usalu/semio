//! ↩️ Inverse for `UpdateLayerTransform` — the OLD transform captured from BASE.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::UpdateLayerTransform, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::update_layer_transform(payload.layer_id.clone(), layer_base(layer).transform.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
