//! 📋️ `set-snapshot` — one rule of CC1's conformance filter, authored as its own mutation leaf.
//! The class-neutral edit is performed by the shared ladder module; this file only names the axis and
//! routes to it, so each rule has ONE implementation and six class callers.

use crate::artifacts::step::schema::diff::StepDiff;
use crate::artifacts::step::standards::v_ap214::subsets::cc1::schema::mutations::StepCc1Mutation;
use crate::artifacts::step::StepSnapshot;
use protocol::command::DiffAlgebra;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: StepSnapshot,
}

impl protocol::MutationKind<StepSnapshot, StepCc1Mutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepCc1Mutation as protocol::Mutation<StepSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepCc1Mutation> {
        vec![StepCc1Mutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })]
    }
    fn label(&self) -> String {
        format!("Set the whole CC1 snapshot")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
