//! 🗑️ Text wire record and conversions owned by the direct `delete-step` leaf.

use crate::artifacts::imperative::schema::mutations::binary::ImperativeMutationDsl;
use crate::artifacts::imperative::schema::mutations::ImperativeMutation;
use crate::artifacts::imperative::PathRef;

pub const TEXT_OPCODE: &str = "delete-step";

//#region 📝️WireRecord
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(keyword = "delete-step")]
pub(crate) struct DeleteStepText {
    owner: Option<String>,
    slot: Option<String>,
    id: String,
}

pub(crate) fn to_dsl(operation: &ImperativeMutation) -> Option<ImperativeMutationDsl> {
    if let ImperativeMutation::DeleteStep(payload) = operation {
        Some(ImperativeMutationDsl::DeleteStep(DeleteStepText { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), id: payload.id.clone() }))
    } else {
        None
    }
}

pub(crate) fn from_dsl(operation: ImperativeMutationDsl) -> Result<ImperativeMutation, ImperativeMutationDsl> {
    if let ImperativeMutationDsl::DeleteStep(payload) = operation {
        Ok(super::delete_step(PathRef { owner: payload.owner, slot: payload.slot }, payload.id))
    } else {
        Err(operation)
    }
}
//#endregion 📝️WireRecord
