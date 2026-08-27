//! 🔧 Text wire record and conversions owned by the direct `edit-step-params` leaf.

use crate::artifacts::imperative::dsl::{dictionary_to_value_dsl_map, value_dsl_map_to_dictionary, ValueDsl};
use crate::artifacts::imperative::schema::mutations::binary::ImperativeMutationDsl;
use crate::artifacts::imperative::schema::mutations::ImperativeMutation;
use crate::artifacts::imperative::PathRef;
use std::collections::BTreeMap;

pub const TEXT_OPCODE: &str = "edit-step-params";

//#region 📝️WireRecord
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(keyword = "edit-step-params")]
pub(crate) struct EditStepParamsText {
    owner: Option<String>,
    slot: Option<String>,
    id: String,
    params: BTreeMap<String, ValueDsl>,
}

pub(crate) fn to_dsl(operation: &ImperativeMutation) -> Option<ImperativeMutationDsl> {
    if let ImperativeMutation::EditStepParams(payload) = operation {
        Some(ImperativeMutationDsl::EditStepParams(EditStepParamsText { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), id: payload.id.clone(), params: dictionary_to_value_dsl_map(&payload.new_params) }))
    } else {
        None
    }
}

pub(crate) fn from_dsl(operation: ImperativeMutationDsl) -> Result<ImperativeMutation, ImperativeMutationDsl> {
    if let ImperativeMutationDsl::EditStepParams(payload) = operation {
        Ok(super::edit_step_params(PathRef { owner: payload.owner, slot: payload.slot }, payload.id, value_dsl_map_to_dictionary(&payload.params)))
    } else {
        Err(operation)
    }
}
//#endregion 📝️WireRecord
