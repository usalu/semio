//! 🔀 Direct Imperative mutation — `ReorderSteps` repositions an id-keyed step within its `PathRef`'s
//! step list (never spatial — see `📓️taxonomy.md`'s `reorder` row).
use crate::artifacts::procedure::diff::ProcedureDiff;
use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::{ProcedureSnapshot, PathRef};

//#region 🔖️Mutation
/// 🔀 `reorder-steps` payload — FINAL-state target index for `id` within its sibling list.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReorderSteps {
    pub path_ref: PathRef,
    pub id: String,
    pub to_index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn reorder_steps(path_ref: PathRef, id: String, to_index: usize) -> ProcedureMutation {
    ProcedureMutation::ReorderSteps(ReorderSteps { path_ref, id, to_index })
}

impl protocol::MutationKind<ProcedureSnapshot, ProcedureMutation> for ReorderSteps {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "steps", kind: "reorder-steps", record: "ReorderedSteps" };

    fn diff(&self, base: &ProcedureSnapshot) -> protocol::MutationOutcome<ProcedureDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProcedureSnapshot) -> Vec<ProcedureMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder step \"{}\" to position {}", self.id, self.to_index)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
