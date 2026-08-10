//! Draw mutation — `DuplicateLayer` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `DuplicateLayer` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateLayer {
    pub layer_id: String,
}

pub fn duplicate_layer(layer_id: String) -> DrawMutation {
    DrawMutation::DuplicateLayer { layer_id }
}

pub fn apply(doc: &mut DrawSnapshot, layer_id: &str) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::DuplicateLayer { layer_id: layer_id.into() });
}
//#endregion 🔖️Mutation
