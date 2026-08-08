//! Draw mutation — `ReorderLayer` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `ReorderLayer` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderLayer {
    pub layer_id: String,
    pub parent_id: Option<String>,
    pub index: usize,
}

pub fn reorder_layer(layer_id: String, parent_id: Option<String>, index: usize) -> DrawMutation {
    DrawMutation::ReorderLayer { layer_id, parent_id, index }
}

pub fn apply(doc: &mut DrawDocument, layer_id: &str, parent_id: &Option<String>, index: usize) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::ReorderLayer { layer_id: layer_id.into(), parent_id: parent_id.clone(), index });
}
//#endregion 🔖️Mutation
