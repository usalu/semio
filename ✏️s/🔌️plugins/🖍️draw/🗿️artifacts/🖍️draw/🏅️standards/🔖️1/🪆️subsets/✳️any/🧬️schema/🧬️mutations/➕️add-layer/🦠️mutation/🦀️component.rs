//! Draw mutation — `AddLayer` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `AddLayer` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLayer {
    pub parent_id: Option<String>,
    pub index: Option<usize>,
    pub layer: Box<crate::artifacts::draw::DrawLayerNode>,
}

pub fn add_layer(parent_id: Option<String>, index: Option<usize>, layer: crate::artifacts::draw::DrawLayerNode) -> DrawMutation {
    DrawMutation::AddLayer { parent_id, index, layer: Box::new(layer) }
}

pub fn apply(doc: &mut DrawSnapshot, parent_id: &Option<String>, index: Option<usize>, layer: &crate::artifacts::draw::DrawLayerNode) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::AddLayer { parent_id: parent_id.clone(), index, layer: Box::new(layer.clone()) });
}
//#endregion 🔖️Mutation
