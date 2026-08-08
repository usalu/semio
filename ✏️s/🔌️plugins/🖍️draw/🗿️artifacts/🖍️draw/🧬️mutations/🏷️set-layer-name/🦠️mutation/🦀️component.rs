//! Draw mutation — `SetLayerName` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetLayerName` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLayerName {
    pub layer_id: String,
    pub name: String,
}

pub fn set_layer_name(layer_id: String, name: String) -> DrawMutation {
    DrawMutation::SetLayerName { layer_id, name }
}

pub fn apply(doc: &mut DrawDocument, layer_id: &str, name: &str) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetLayerName { layer_id: layer_id.into(), name: name.into() });
}
//#endregion 🔖️Mutation
