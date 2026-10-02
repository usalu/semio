//! 📸️ Whole BMP snapshot replacement for schema-driven editing.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::BmpMutation;
use crate::BmpSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: BmpSnapshot,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<BmpSnapshot, BmpMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<<BmpMutation as Mutation<BmpSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<BmpDiff as DiffAlgebra<BmpSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        vec![BmpMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen")
    }
    fn target(&self) -> Vec<String> { Vec::new() }
}
