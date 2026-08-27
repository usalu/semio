//! 🌱 Text wire record and conversions owned by the direct `create-step` leaf.

use crate::artifacts::imperative::dsl::{step_node_dsl_to_step, step_to_step_node_dsl, StepNodeDsl};
use crate::artifacts::imperative::schema::mutations::binary::ImperativeMutationDsl;
use crate::artifacts::imperative::schema::mutations::ImperativeMutation;
use crate::artifacts::imperative::PathRef;

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

pub(crate) fn to_dsl(operation: &ImperativeMutation) -> Option<ImperativeMutationDsl> {
    if let ImperativeMutation::CreateStep(payload) = operation {
        Some(ImperativeMutationDsl::CreateStep(CreateStepText { owner: payload.path_ref.owner.clone(), slot: payload.path_ref.slot.clone(), item: Box::new(step_to_step_node_dsl(&payload.step)) }))
    } else {
        None
    }
}

pub(crate) fn from_dsl(operation: ImperativeMutationDsl) -> Result<ImperativeMutation, ImperativeMutationDsl> {
    if let ImperativeMutationDsl::CreateStep(payload) = operation {
        Ok(super::create_step(PathRef { owner: payload.owner, slot: payload.slot }, step_node_dsl_to_step(*payload.item)))
    } else {
        Err(operation)
    }
}
//#endregion 📝️WireRecord
