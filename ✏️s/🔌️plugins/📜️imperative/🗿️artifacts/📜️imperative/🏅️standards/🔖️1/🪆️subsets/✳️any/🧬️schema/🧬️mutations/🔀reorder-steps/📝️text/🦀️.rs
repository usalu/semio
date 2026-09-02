//! 🔀 Text wire record and conversions owned by the direct `reorder-steps` leaf.

use crate::artifacts::imperative::schema::mutations::binary::ImperativeMutationDsl;
use crate::artifacts::imperative::schema::mutations::ImperativeMutation;
use crate::artifacts::imperative::PathRef;

pub const TEXT_OPCODE: &str = "reorder-steps";

//#region 📝️WireRecord
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(keyword = "reorder-steps")]
pub(crate) struct ReorderStepsText {
    owner: Option<String>,
    slot: Option<String>,
    id: String,
    #[dsl(key = "to")]
    to_index: usize,
}

pub(crate) fn to_dsl(operation: &ImperativeMutation) -> Option<ImperativeMutationDsl> {
    if let ImperativeMutation::ReorderSteps(payload) = operation {
        Some(ImperativeMutationDsl::ReorderSteps(ReorderStepsText { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), id: payload.id.clone(), to_index: payload.to_index }))
    } else {
        None
    }
}

pub(crate) fn from_dsl(operation: ImperativeMutationDsl) -> Result<ImperativeMutation, ImperativeMutationDsl> {
    if let ImperativeMutationDsl::ReorderSteps(payload) = operation {
        Ok(super::reorder_steps(PathRef { owner: payload.owner, slot: payload.slot }, payload.id, payload.to_index))
    } else {
        Err(operation)
    }
}
//#endregion 📝️WireRecord
