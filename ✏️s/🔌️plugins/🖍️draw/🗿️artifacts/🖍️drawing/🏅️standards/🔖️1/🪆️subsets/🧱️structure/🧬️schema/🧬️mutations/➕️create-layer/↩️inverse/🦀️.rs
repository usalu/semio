//! ↩️ Inverse for `CreateLayer` — always a `delete-layer` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).
use crate::mutations::DrawingMutation;
use crate::schema::layer_id;
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateLayer, _base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    vec![crate::mutations::delete_layer(layer_id(&payload.layer).to_string())]
}
//#endregion 🔖️Inverse
