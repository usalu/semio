//! 🌱 Text wire record and conversions owned by the direct `create-step` leaf.

use crate::artifacts::procedure::dsl::{step_node_dsl_to_step, step_to_step_node_dsl, StepNodeDsl};
use crate::artifacts::procedure::schema::mutations::binary::ProcedureMutationDsl;
use crate::artifacts::procedure::schema::mutations::ProcedureMutation;
use crate::artifacts::procedure::PathRef;

pub const TEXT_OPCODE: &str = "create-step";

//#region 📝️WireRecord
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(keyword = "create-step")]
pub(crate) struct CreateStepText {
    owner: Option<String>,
    slot: Option<String>,
    #[dsl(statements)]
    item: Box<StepNodeDsl>,
}

pub(crate) fn to_dsl(operation: &ProcedureMutation) -> Option<ProcedureMutationDsl> {
    if let ProcedureMutation::CreateStep(payload) = operation {
        Some(ProcedureMutationDsl::CreateStep(CreateStepText { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), item: Box::new(step_to_step_node_dsl(&payload.step)) }))
    } else {
        None
    }
}

pub(crate) fn from_dsl(operation: ProcedureMutationDsl) -> Result<ProcedureMutation, ProcedureMutationDsl> {
    if let ProcedureMutationDsl::CreateStep(payload) = operation {
        Ok(super::create_step(PathRef { owner: payload.owner, slot: payload.slot }, step_node_dsl_to_step(*payload.item)))
    } else {
        Err(operation)
    }
}
//#endregion 📝️WireRecord
