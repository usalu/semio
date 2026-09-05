//! ↩️ Inverse for `SetLayerOpacity` — reconstructed from BASE state.
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::{find_drawing_layer, layer_base};
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::SetLayerOpacity, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_opacity(payload.layer_id.clone(), layer_base(layer).opacity)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
