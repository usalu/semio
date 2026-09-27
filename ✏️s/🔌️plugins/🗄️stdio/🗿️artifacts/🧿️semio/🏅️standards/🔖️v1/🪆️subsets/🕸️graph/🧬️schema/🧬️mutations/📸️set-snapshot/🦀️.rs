//! 📸️ Whole SemioGraphSnapshot replacement for schema-driven editing.

use super::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SemioGraphSnapshot,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as Mutation<SemioGraphSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SemioGraphDiff as DiffAlgebra<SemioGraphSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        vec![SemioGraphMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
