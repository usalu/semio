//! 🗑️ Imperative mutation — `DeleteStep`: removes an id-keyed step (its `bodies` cascade goes
//! with it — no separate reconnection logic needed).
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::{ImperativeSnapshot, PathRef};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-step` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStep {
    pub path_ref: PathRef,
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_step(path_ref: PathRef, id: String) -> ImperativeMutation {
    ImperativeMutation::DeleteStep(DeleteStep { path_ref, id })
}

impl protocol::MutationKind<ImperativeSnapshot, ImperativeMutation> for DeleteStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    async fn diff(&self, base: &ImperativeSnapshot) -> protocol::MutationOutcome<ImperativeDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete step \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
