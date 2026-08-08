//! Draw mutation — `SetStroke` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetStroke` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStroke {
    pub layer_id: String,
    pub stroke: Option<crate::artifacts::draw::StrokeStyle>,
}

pub fn set_stroke(layer_id: String, stroke: Option<crate::artifacts::draw::StrokeStyle>) -> DrawMutation {
    DrawMutation::SetStroke { layer_id, stroke }
}

pub fn apply(doc: &mut DrawDocument, layer_id: &str, stroke: &Option<crate::artifacts::draw::StrokeStyle>) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetStroke { layer_id: layer_id.into(), stroke: stroke.clone() });
}
//#endregion 🔖️Mutation
