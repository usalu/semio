//! ↩️ Inverse for `CreateLayer` — always a `delete-layer` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::schema::layer_id;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateLayer, _base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    vec![crate::artifacts::drawing::mutations::delete_layer::mutation::delete_layer(layer_id(&payload.layer).to_string())]
}
//#endregion 🔖️Inverse
