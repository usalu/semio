//! 📸️ Whole JpgSnapshot replacement for schema-driven editing.

use super::JpgMutation;
use crate::schema::diff::JpgDiff;
use crate::JpgSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: JpgSnapshot,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<JpgSnapshot, JpgMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<<JpgMutation as Mutation<JpgSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<JpgDiff as DiffAlgebra<JpgSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        vec![JpgMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
