//! Draw mutation — `SetLayerTransform` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetLayerTransform` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLayerTransform {
    pub layer_id: String,
    pub transform: crate::artifacts::draw::DrawTransform,
}

pub fn set_layer_transform(layer_id: String, transform: crate::artifacts::draw::DrawTransform) -> DrawMutation {
    DrawMutation::SetLayerTransform { layer_id, transform }
}

pub fn apply(doc: &mut DrawSnapshot, layer_id: &str, transform: &crate::artifacts::draw::DrawTransform) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetLayerTransform { layer_id: layer_id.into(), transform: transform.clone() });
}
//#endregion 🔖️Mutation
