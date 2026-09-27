//! 📸️ Whole PNG snapshot replacement for schema-driven editing.

use crate::schema::diff::PngDiff;
use crate::schema::mutations::PngMutation;
use crate::PngSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: PngSnapshot,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<PngSnapshot, PngMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<<PngMutation as Mutation<PngSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<PngDiff as DiffAlgebra<PngSnapshot>>::between(base, &self.snapshot))
    }

    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        vec![PngMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
