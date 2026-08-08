//! Draw mutation — `SetLayerOpacity` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetLayerOpacity` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLayerOpacity {
    pub layer_id: String,
    pub opacity: f64,
}

pub fn set_layer_opacity(layer_id: String, opacity: f64) -> DrawMutation {
    DrawMutation::SetLayerOpacity { layer_id, opacity }
}

pub fn apply(doc: &mut DrawDocument, layer_id: &str, opacity: f64) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetLayerOpacity { layer_id: layer_id.into(), opacity });
}
//#endregion 🔖️Mutation
