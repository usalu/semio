//! Draw mutation — `SetLayerVisible` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetLayerVisible` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLayerVisible {
    pub layer_id: String,
    pub visible: bool,
}

pub fn set_layer_visible(layer_id: String, visible: bool) -> DrawMutation {
    DrawMutation::SetLayerVisible { layer_id, visible }
}

pub fn apply(doc: &mut DrawDocument, layer_id: &str, visible: bool) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetLayerVisible { layer_id: layer_id.into(), visible });
}
//#endregion 🔖️Mutation
