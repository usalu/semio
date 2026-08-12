//! ↩️ Inverse for `CreateLayer` — always a `delete-layer` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).
use crate::artifacts::draw::schema::layer_id;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateLayer, _base: &DrawSnapshot) -> Vec<DrawMutation> {
    vec![crate::artifacts::draw::mutations::delete_layer::mutation::delete_layer(layer_id(&payload.layer).to_string())]
}
//#endregion 🔖️Inverse
