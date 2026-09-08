//! 🗑️ Text wire record and conversions owned by the direct `delete-step` leaf.

use crate::schema::mutations::binary::ProcedureMutationDsl;
use crate::schema::mutations::ProcedureMutation;
use crate::PathRef;

pub const TEXT_OPCODE: &str = "delete-step";

//#region 📝️WireRecord
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(keyword = "delete-step")]
pub(crate) struct DeleteStepText {
    owner: Option<String>,
    slot: Option<String>,
    id: String,
}

pub(crate) fn to_dsl(operation: &ProcedureMutation) -> Option<ProcedureMutationDsl> {
    if let ProcedureMutation::DeleteStep(payload) = operation {
        Some(ProcedureMutationDsl::DeleteStep(DeleteStepText { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), id: payload.id.clone() }))
    } else {
        None
    }
}

pub(crate) fn from_dsl(operation: ProcedureMutationDsl) -> Result<ProcedureMutation, ProcedureMutationDsl> {
    if let ProcedureMutationDsl::DeleteStep(payload) = operation {
        Ok(super::delete_step(PathRef { owner: payload.owner, slot: payload.slot }, payload.id))
    } else {
        Err(operation)
    }
}
//#endregion 📝️WireRecord
