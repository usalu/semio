//! Draw mutation — `SetFill` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetFill` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFill {
    pub layer_id: String,
    pub fill: Option<crate::artifacts::draw::FillStyle>,
}

pub fn set_fill(layer_id: String, fill: Option<crate::artifacts::draw::FillStyle>) -> DrawMutation {
    DrawMutation::SetFill { layer_id, fill }
}

pub fn apply(doc: &mut DrawDocument, layer_id: &str, fill: &Option<crate::artifacts::draw::FillStyle>) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetFill { layer_id: layer_id.into(), fill: fill.clone() });
}
//#endregion 🔖️Mutation
