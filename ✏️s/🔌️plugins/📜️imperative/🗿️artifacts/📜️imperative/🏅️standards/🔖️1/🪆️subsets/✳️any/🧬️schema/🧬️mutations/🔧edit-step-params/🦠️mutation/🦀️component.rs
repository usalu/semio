//! 🔧 Imperative mutation — `EditStepParams`: replaces a step's authored `params` dictionary
//! wholesale (a full value replace, never a merge — `apply_steps_delta`'s `patched` handling does
//! `step.params = entry.patch.clone()`).
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, PathRef};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔧 `edit-step-params` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditStepParams {
    pub path_ref: PathRef,
    pub id: String,
    pub new_params: Dictionary,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_step_params(path_ref: PathRef, id: String, new_params: Dictionary) -> ImperativeMutation {
    ImperativeMutation::EditStepParams(EditStepParams { path_ref, id, new_params })
}

impl protocol::MutationKind<ImperativeSnapshot, ImperativeMutation> for EditStepParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "step", kind: "edit-step-params", record: "EditedStepParams" };

    fn diff(&self, base: &ImperativeSnapshot) -> protocol::MutationOutcome<ImperativeDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
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
