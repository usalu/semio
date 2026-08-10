//! Draw mutation — `SetLayerLocked` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetLayerLocked` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLayerLocked {
    pub layer_id: String,
    pub locked: bool,
}

pub fn set_layer_locked(layer_id: String, locked: bool) -> DrawMutation {
    DrawMutation::SetLayerLocked { layer_id, locked }
}

pub fn apply(doc: &mut DrawSnapshot, layer_id: &str, locked: bool) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetLayerLocked { layer_id: layer_id.into(), locked });
}
//#endregion 🔖️Mutation
