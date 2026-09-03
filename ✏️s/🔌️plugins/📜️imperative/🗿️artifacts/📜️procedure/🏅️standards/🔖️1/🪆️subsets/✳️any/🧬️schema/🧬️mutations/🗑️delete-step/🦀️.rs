//! 🗑️ Direct Imperative mutation — `DeleteStep` removes an id-keyed step (its `bodies` cascade goes
//! with it — no separate reconnection logic needed).
use crate::artifacts::procedure::diff::ProcedureDiff;
use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::{ProcedureSnapshot, PathRef};

//#region 🔖️Mutation
/// 🗑️ `delete-step` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteStep {
    pub path_ref: PathRef,
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_step(path_ref: PathRef, id: String) -> ProcedureMutation {
    ProcedureMutation::DeleteStep(DeleteStep { path_ref, id })
}

impl protocol::MutationKind<ProcedureSnapshot, ProcedureMutation> for DeleteStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "step", kind: "delete-step", record: "DeletedStep" };

    fn diff(&self, base: &ProcedureSnapshot) -> protocol::MutationOutcome<ProcedureDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProcedureSnapshot) -> Vec<ProcedureMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete step \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
