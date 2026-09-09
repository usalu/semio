//! 📋️ `set-snapshot` — one axis of this conformance class, authored as its own mutation leaf.
//! The class-neutral edit is performed by the shared ladder module; this file names the axis and
//! routes to it, so each rule has ONE implementation and every class calls it.

use crate::schema::diff::StepDiff;
use crate::standards::v_ap214::subsets::cc6::schema::mutations::StepCc6Mutation;
use crate::StepSnapshot;
use protocol::command::DiffAlgebra;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: StepSnapshot,
}

impl protocol::MutationKind<StepSnapshot, StepCc6Mutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepCc6Mutation as protocol::Mutation<StepSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepCc6Mutation> {
        vec![StepCc6Mutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })]
    }
    fn label(&self) -> String {
        "Set the whole CC6 snapshot".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
