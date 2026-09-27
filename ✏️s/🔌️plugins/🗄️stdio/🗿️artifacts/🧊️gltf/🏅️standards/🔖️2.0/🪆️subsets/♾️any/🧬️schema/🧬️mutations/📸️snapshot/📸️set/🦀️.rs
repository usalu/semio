//! 📸️ Whole GltfSnapshot replacement for schema-driven editing.

use super::GltfMutation;
use crate::schema::diff::GltfDiff;
use crate::GltfSnapshot;
use protocol::{DiffAlgebra, Mutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub snapshot: GltfSnapshot,
}

impl protocol::MutationKind<GltfSnapshot, GltfMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };
    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<<GltfMutation as Mutation<GltfSnapshot>>::Diff> {
        protocol::MutationOutcome::new(<GltfDiff as DiffAlgebra<GltfSnapshot>>::between(base, &self.snapshot))
    }
    fn inverse(&self, base: &GltfSnapshot) -> Vec<GltfMutation> {
        vec![GltfMutation::SetSnapshot(Self { snapshot: base.clone() })]
    }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Set snapshot", "Momentaufnahme setzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}
