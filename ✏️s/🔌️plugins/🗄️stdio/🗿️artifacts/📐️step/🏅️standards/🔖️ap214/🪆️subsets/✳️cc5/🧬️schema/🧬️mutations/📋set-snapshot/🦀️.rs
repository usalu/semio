//! 📋️ `set-snapshot` — one axis of this conformance class, authored as its own mutation leaf.
//! The class-neutral edit is performed by the shared ladder module; this file names the axis and
//! routes to it, so each rule has ONE implementation and every class calls it.

use crate::artifacts::step::StepSnapshot;
use crate::artifacts::step::schema::diff::StepDiff;
use protocol::command::DiffAlgebra;
use crate::artifacts::step::standards::v_ap214::subsets::cc5::schema::mutations::{StepCc5Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: StepSnapshot,
}

impl protocol::MutationKind<StepSnapshot, StepCc5Mutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<<StepCc5Mutation as protocol::Mutation<StepSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<StepDiff as DiffAlgebra<StepSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &StepSnapshot) -> Vec<StepCc5Mutation> {
        vec![StepCc5Mutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })]
    }
    fn label(&self) -> String {
        format!("Set the whole CC5 snapshot")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
