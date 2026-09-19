//! 📜️ Imperative semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::diff::ProcedureDiff;
use crate::ProcedureSnapshot;

pub use super::create_step::{create_step, CreateStep};
pub use super::delete_step::{delete_step, DeleteStep};
pub use super::edit_step_params::{edit_step_params, EditStepParams};
pub use super::reorder_steps::{reorder_steps, ReorderSteps};
pub use crate::standards::v1::subsets::any::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Semantic Imperative document mutation vocabulary.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ProcedureSnapshot, diff = ProcedureDiff, schema = "imperative.imperative", retire_cold = retire_procedure_mutation)]
pub enum ProcedureMutation {
    CreateStep(CreateStep),
    DeleteStep(DeleteStep),
    ReorderSteps(ReorderSteps),
    EditStepParams(EditStepParams),
}

/// 🧊️ Cold disposal of one operation: created steps and edited params own neural dictionaries.
fn retire_procedure_mutation(mutation: ProcedureMutation) {
    use neural_engine::ColdRetire;
    match mutation {
        ProcedureMutation::CreateStep(value) => value.step.retire_cold(),
        ProcedureMutation::EditStepParams(value) => value.new_params.retire_cold(),
        ProcedureMutation::DeleteStep(_) | ProcedureMutation::ReorderSteps(_) => {}
    }
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
