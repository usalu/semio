//! 🧬️ Direct move-node-child mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::modules::mutation_support::structure_geometry::checked_index;
pub const ID: &str = "s.stdio.gltf.mutation.move-node-child.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveNodeChildPayload { pub parent: usize, pub child: usize, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveNodeChildPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?; let children = &base.document.nodes[payload.parent].children; let index = children.iter().position(|child| *child == payload.child).ok_or_else(|| reject("gltf.mutation.relation-absent", "document/nodes/children", "child is not linked to parent"))?; checked_index(payload.position, children.len(), "document/nodes/children")?; if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/nodes/children", "destination equals source")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveNodeChildPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let children = &mut next.document.nodes[payload.parent].children; let index = children.iter().position(|child| *child == payload.child).expect("validated child"); let child = children.remove(index); children.insert(payload.position, child); Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveNodeChildMutation {
    Apply(GltfMoveNodeChildPayload),
    Restore(Box<crate::artifacts::gltf::schema::diff::GltfDiff>),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveNodeChildMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node-child", kind: "move-node-child", record: "MovedNodeChild" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::artifacts::gltf::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => { match apply(payload, base) { Ok(next) => protocol::MutationOutcome::new(<crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)), Err(error) => rejection_outcome(&error.code, &error.path, error.detail) } }
            Self::Restore(diff) => match protocol::MutationDiff::apply(diff.as_ref(), base) {
                Ok(_) => protocol::MutationOutcome::new(diff.as_ref().clone()),
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
        vec![super::GltfMutation::MoveNodeChild(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Move Node Child".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["move-node-child".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<MoveNodeChildMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-node-child");
    }
}
//#endregion 🧪️Tests
