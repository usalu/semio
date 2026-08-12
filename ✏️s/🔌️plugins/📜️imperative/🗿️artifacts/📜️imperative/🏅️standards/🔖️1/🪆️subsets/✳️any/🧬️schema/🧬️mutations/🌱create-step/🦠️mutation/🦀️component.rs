//! 🌱 Imperative mutation — `CreateStep`: brings a new id-keyed `Step` into existence at a
//! `PathRef` (root path, or a nested `control.*` step's body slot).
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::{ImperativeSnapshot, PathRef, Step};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱 `create-step` payload — the full step (its own `bodies` cascade travels with it, no
/// separate reconnection logic needed).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStep {
    pub path_ref: PathRef,
    pub step: Step,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_step(path_ref: PathRef, step: Step) -> ImperativeMutation {
    ImperativeMutation::CreateStep(CreateStep { path_ref, step })
}

impl protocol::MutationKind<ImperativeSnapshot, ImperativeMutation> for CreateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "step", kind: "create-step", record: "CreatedStep" };

    fn diff(&self, base: &ImperativeSnapshot) -> ImperativeDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create step \"{}\"", self.step.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.step.id.clone()]
    }
}
//#endregion 🔖️Mutation
