//! Draw mutation — `SetTraceParams` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetTraceParams` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTraceParams {
    pub layer_id: String,
    pub params: crate::artifacts::draw::DrawTraceParams,
}

pub fn set_trace_params(layer_id: String, params: crate::artifacts::draw::DrawTraceParams) -> DrawMutation {
    DrawMutation::SetTraceParams { layer_id, params }
}

pub fn apply(doc: &mut DrawSnapshot, layer_id: &str, params: &crate::artifacts::draw::DrawTraceParams) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetTraceParams { layer_id: layer_id.into(), params: params.clone() });
}
//#endregion 🔖️Mutation
