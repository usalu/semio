//! 🌱 Direct Imperative mutation — `CreateStep` brings a new id-keyed `Step` into existence at a
//! `PathRef` (root path, or a nested `control.*` step's body slot).
use crate::artifacts::procedure::diff::ProcedureDiff;
use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::{ProcedureSnapshot, PathRef, Step};

//#region 🔖️Mutation
/// 🌱 `create-step` payload — the full step (its own `bodies` cascade travels with it, no
/// separate reconnection logic needed).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateStep {
    pub path_ref: PathRef,
    pub step: Step,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_step(path_ref: PathRef, step: Step) -> ProcedureMutation {
    ProcedureMutation::CreateStep(CreateStep { path_ref, step })
}

impl protocol::MutationKind<ProcedureSnapshot, ProcedureMutation> for CreateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "step", kind: "create-step", record: "CreatedStep" };

    fn diff(&self, base: &ProcedureSnapshot) -> protocol::MutationOutcome<ProcedureDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProcedureSnapshot) -> Vec<ProcedureMutation> {
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
