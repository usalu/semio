//! Draw mutation — `SetLayerBlendMode` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetLayerBlendMode` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLayerBlendMode {
    pub layer_id: String,
    pub blend_mode: String,
}

pub fn set_layer_blend_mode(layer_id: String, blend_mode: String) -> DrawMutation {
    DrawMutation::SetLayerBlendMode { layer_id, blend_mode }
}

pub fn apply(doc: &mut DrawDocument, layer_id: &str, blend_mode: &str) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetLayerBlendMode { layer_id: layer_id.into(), blend_mode: blend_mode.into() });
}
//#endregion 🔖️Mutation
