//! ↩️ Inverse for `RenameLayer` — the OLD name looked up from BASE, never a captured id.
use crate::mutations::DrawingMutation;
use crate::schema::{find_drawing_layer, layer_base};
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RenameLayer, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match find_drawing_layer(base, &payload.layer_id) {
        Some(layer) => vec![super::mutation::rename_layer(payload.layer_id.clone(), layer_base(layer).name.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
