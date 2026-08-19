//! 🔀 Imperative mutation — `ReorderSteps`: repositions an id-keyed step within its `PathRef`'s
//! step list (never spatial — see `📓️taxonomy.md`'s `reorder` row).
use crate::artifacts::imperative::diff::ImperativeDiff;
use crate::artifacts::imperative::mutations::ImperativeMutation;
use crate::artifacts::imperative::{ImperativeSnapshot, PathRef};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔀 `reorder-steps` payload — FINAL-state target index for `id` within its sibling list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderSteps {
    pub path_ref: PathRef,
    pub id: String,
    pub to_index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn reorder_steps(path_ref: PathRef, id: String, to_index: usize) -> ImperativeMutation {
    ImperativeMutation::ReorderSteps(ReorderSteps { path_ref, id, to_index })
}

impl protocol::MutationKind<ImperativeSnapshot, ImperativeMutation> for ReorderSteps {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "steps", kind: "reorder-steps", record: "ReorderedSteps" };

    async fn diff(&self, base: &ImperativeSnapshot) -> protocol::MutationOutcome<ImperativeDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ImperativeSnapshot) -> Vec<ImperativeMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Reorder step \"{}\" to position {}", self.id, self.to_index)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
