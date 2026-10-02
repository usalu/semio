//! 📸️ Whole SemioKitSnapshot replacement for schema-driven editing.

use super::SemioKitMutation;
use crate::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: SemioKitSnapshot,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as Mutation<SemioKitSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<SemioKitDiff as DiffAlgebra<SemioKitSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        vec![SemioKitMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
