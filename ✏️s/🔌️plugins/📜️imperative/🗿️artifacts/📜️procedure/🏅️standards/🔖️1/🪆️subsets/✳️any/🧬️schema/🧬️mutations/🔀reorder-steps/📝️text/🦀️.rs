//! 🔀 Text wire record and conversions owned by the direct `reorder-steps` leaf.

use crate::artifacts::procedure::schema::mutations::binary::ProcedureMutationDsl;
use crate::artifacts::procedure::schema::mutations::ProcedureMutation;
use crate::artifacts::procedure::PathRef;

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

pub(crate) fn to_dsl(operation: &ProcedureMutation) -> Option<ProcedureMutationDsl> {
    if let ProcedureMutation::ReorderSteps(payload) = operation {
        Some(ProcedureMutationDsl::ReorderSteps(ReorderStepsText { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), id: payload.id.clone(), to_index: payload.to_index }))
    } else {
        None
    }
}

pub(crate) fn from_dsl(operation: ProcedureMutationDsl) -> Result<ProcedureMutation, ProcedureMutationDsl> {
    if let ProcedureMutationDsl::ReorderSteps(payload) = operation {
        Ok(super::reorder_steps(PathRef { owner: payload.owner, slot: payload.slot }, payload.id, payload.to_index))
    } else {
        Err(operation)
    }
}
//#endregion 📝️WireRecord
