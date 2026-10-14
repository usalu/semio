//! ↩️ Inverse for `SetLayerVisible` — real logic reconstructing the undo from BASE state, never
//! from post-state. Missing target ⇒ `Vec::new()` (the semantic replacement for `NoMutation`).

use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::set_layer_visible::mutation::SetLayerVisible>>(payload: &super::mutation::SetLayerVisible, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::set_layer_visible(payload.layer_id.clone(), layer_base(layer).visible)],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
