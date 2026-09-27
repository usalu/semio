//! 📸️ Whole SemioDrawingSnapshot replacement for schema-driven editing.

use super::SemioDrawingMutation;
use crate::standards::v1::subsets::drawing::schema::diff::SemioDrawingDiff;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SemioDrawingSnapshot,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SemioDrawingDiff as DiffAlgebra<SemioDrawingSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        vec![SemioDrawingMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
