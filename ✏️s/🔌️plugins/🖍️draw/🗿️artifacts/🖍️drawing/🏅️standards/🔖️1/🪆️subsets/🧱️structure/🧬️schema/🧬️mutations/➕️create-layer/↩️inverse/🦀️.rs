//! ↩️ Inverse for `CreateLayer` — always a `delete-layer` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).

use semio_s_artifact_draw_drawing_model::layer_id;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::delete_layer::mutation::DeleteLayer>>(payload: &super::mutation::CreateLayer, _base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(vec![crate::delete_layer::mutation::delete_layer(layer_id(&payload.layer).to_string().into())])
}
//#endregion 🔖️Inverse
