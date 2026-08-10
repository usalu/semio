//! Draw mutation — `SetBooleanOperation` payload + builder + apply.
use crate::artifacts::draw::mutations::{apply_draw_edit_mutation, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji `SetBooleanOperation` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBooleanOperation {
    pub layer_id: String,
    pub boolean_operation: String,
}

pub fn set_boolean_operation(layer_id: String, boolean_operation: String) -> DrawMutation {
    DrawMutation::SetBooleanOperation { layer_id, boolean_operation }
}

pub fn apply(doc: &mut DrawSnapshot, layer_id: &str, boolean_operation: &str) {
    *doc = apply_draw_edit_mutation(doc, &DrawMutation::SetBooleanOperation { layer_id: layer_id.into(), boolean_operation: boolean_operation.into() });
}
//#endregion 🔖️Mutation
