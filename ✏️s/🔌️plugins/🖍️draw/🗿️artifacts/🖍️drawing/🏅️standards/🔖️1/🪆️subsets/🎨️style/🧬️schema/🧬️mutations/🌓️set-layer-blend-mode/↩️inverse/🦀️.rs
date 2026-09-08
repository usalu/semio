//! ↩️ Inverse for `SetLayerBlendMode` — reconstructed from BASE state.
use crate::mutations::DrawingMutation;
use crate::schema::{find_drawing_layer, layer_base};
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetLayerBlendMode, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_blend_mode(payload.layer_id.clone(), layer_base(layer).blend_mode.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
