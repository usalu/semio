//! 🧬️ Direct move-node-parent mutation owner: payload, validation, typed diff, inverse, and outcomes.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.move-node-parent.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct GltfReparentNodePayload { pub parent: usize, pub child: usize, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReparentNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?; checked_index(payload.child, base.document.nodes.len(), "document/nodes")?; if payload.parent == payload.child { return Err(reject("gltf.mutation.node-cycle", "document/nodes", "a node cannot parent itself")); } let mut pending = vec![payload.child]; let mut seen = std::collections::BTreeSet::new(); while let Some(node) = pending.pop() { if node == payload.parent { return Err(reject("gltf.mutation.node-cycle", "document/nodes", "relationship closes a cycle")); } if seen.insert(node) { pending.extend(base.document.nodes[node].children.iter().copied()); } } let length = base.document.nodes[payload.parent].children.iter().filter(|child| **child != payload.child).count(); checked_position(payload.position, length, "document/nodes/children")?; Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfReparentNodePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); for node in &mut next.document.nodes { node.children.retain(|child| *child != payload.child); } for scene in &mut next.document.scenes { scene.nodes.retain(|node| *node != payload.child); } next.document.nodes[payload.parent].children.insert(payload.position, payload.child); Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveNodeParentMutation {
    Apply(GltfReparentNodePayload),
    Restore(crate::artifacts::gltf::schema::diff::GltfDiff),
}

fn rejection_outcome(code: String, path: String, detail: String) -> protocol::MutationOutcome<crate::artifacts::gltf::schema::diff::GltfDiff> {
    let target = path.split('/').filter(|part| !part.is_empty()).map(str::to_string).collect::<Vec<_>>();
    if code.contains("no-observable-change") {
        return protocol::MutationOutcome::new(Default::default()).warn("mutation.no-op", detail);
    }
    if code.contains("duplicate") {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", detail, target);
    }
    if code.contains("out-of-range") || code.contains("missing") || code.contains("not-found") {
        return protocol::MutationOutcome::error("mutation.target-missing", detail, target);
    }
    protocol::MutationOutcome::fatal("mutation.invariant", format!("{code}: {detail}"), target)
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveNodeParentMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node-parent", kind: "move-node-parent", record: "MovedNodeParent" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::artifacts::gltf::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => { match apply(payload, base) { Ok(next) => protocol::MutationOutcome::new(<crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)), Err(error) => rejection_outcome(error.code, error.path, error.detail) } }
            Self::Restore(diff) => match protocol::MutationDiff::apply(diff, base) {
                Ok(_) => protocol::MutationOutcome::new(diff.clone()),
                Err(error) => protocol::MutationOutcome::fatal("mutation.invariant", error.to_string(), error.target),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
        let outcome = <Self as protocol::MutationKind<GltfSnapshot, super::GltfMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || outcome.diff().is_empty_diff() {
            return Vec::new();
        }
        let inverse = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::inverse(outcome.diff(), base);
        vec![super::GltfMutation::MoveNodeParent(Self::Restore(inverse))]
    }

    fn label(&self) -> String {
        "Move Node Parent".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["move-node-parent".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<MoveNodeParentMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-node-parent");
    }
}
//#endregion 🧪️Tests
