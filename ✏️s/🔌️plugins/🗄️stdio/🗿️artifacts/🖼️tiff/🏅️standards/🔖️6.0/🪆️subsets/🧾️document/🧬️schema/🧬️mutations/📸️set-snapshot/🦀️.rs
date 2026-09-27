//! 📸️ Whole TiffSnapshot replacement for schema-driven editing.

use super::TiffMutation;
use crate::schema::diff::TiffDiff;
use crate::TiffSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: TiffSnapshot,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<TiffSnapshot, TiffMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<<TiffMutation as Mutation<TiffSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<TiffDiff as DiffAlgebra<TiffSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffMutation> {
        vec![TiffMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
