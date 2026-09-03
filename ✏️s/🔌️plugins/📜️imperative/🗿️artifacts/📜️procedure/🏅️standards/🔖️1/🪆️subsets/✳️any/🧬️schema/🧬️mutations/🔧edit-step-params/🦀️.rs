//! 🔧 Direct Imperative mutation — `EditStepParams` replaces a step's authored `params` dictionary
//! wholesale (a full value replace, never a merge — `apply_steps_delta`'s `patched` handling does
//! `step.params = entry.patch.clone()`).
use crate::artifacts::procedure::diff::ProcedureDiff;
use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::{Dictionary, ProcedureSnapshot, PathRef};

//#region 🔖️Mutation
/// 🔧 `edit-step-params` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct EditStepParams {
    pub path_ref: PathRef,
    pub id: String,
    pub new_params: Dictionary,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_step_params(path_ref: PathRef, id: String, new_params: Dictionary) -> ProcedureMutation {
    ProcedureMutation::EditStepParams(EditStepParams { path_ref, id, new_params })
}

impl protocol::MutationKind<ProcedureSnapshot, ProcedureMutation> for EditStepParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "step", kind: "edit-step-params", record: "EditedStepParams" };

    fn diff(&self, base: &ProcedureSnapshot) -> protocol::MutationOutcome<ProcedureDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProcedureSnapshot) -> Vec<ProcedureMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Edit step \"{}\" parameters", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
