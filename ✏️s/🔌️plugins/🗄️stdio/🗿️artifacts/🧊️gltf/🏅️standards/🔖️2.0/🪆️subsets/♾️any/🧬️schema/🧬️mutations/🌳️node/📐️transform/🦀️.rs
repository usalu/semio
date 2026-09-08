//! 🧬️ Direct change-node-transform mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::GltfSnapshot;
use crate::schema::modules::mutation_support::top_level::{GltfTopLevelMutationRejection, reject};
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
pub const ID: &str = "s.stdio.gltf.mutation.change-node-transform.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfTransformNodePayload { pub node: usize, pub transform: GltfNodeTransform }
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum GltfNodeTransform { Matrix { matrix: [f64; 16] }, Trs { translation: Option<[f64; 3]>, rotation: Option<[f64; 4]>, scale: Option<[f64; 3]> } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfTransformNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.node, base.document.nodes.len(), "document/nodes")?; let finite = match &payload.transform { GltfNodeTransform::Matrix { matrix } => matrix.iter().all(|value| value.is_finite()), GltfNodeTransform::Trs { translation, rotation, scale } => translation.iter().flatten().chain(rotation.iter().flatten()).chain(scale.iter().flatten()).all(|value| value.is_finite()) }; if !finite { return Err(reject("gltf.mutation.invalid-transform", format!("document/nodes/{}/transform", payload.node), "transform values must be finite")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfTransformNodePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let node = &mut next.document.nodes[payload.node]; match &payload.transform { GltfNodeTransform::Matrix { matrix } => { node.matrix = Some(*matrix); node.translation = None; node.rotation = None; node.scale = None; }, GltfNodeTransform::Trs { translation, rotation, scale } => { node.matrix = None; node.translation = *translation; node.rotation = *rotation; node.scale = *scale; } } Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeNodeTransformMutation {
    Apply(GltfTransformNodePayload),
    Restore(Box<crate::schema::diff::GltfDiff>),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeNodeTransformMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-transform", kind: "change-node-transform", record: "ChangedNodeTransform" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => { match apply(payload, base) { Ok(next) => protocol::MutationOutcome::new(<crate::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)), Err(error) => rejection_outcome(&error.code, &error.path, error.detail) } }
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
        let inverse = <crate::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::inverse(outcome.diff(), base);
        vec![super::GltfMutation::ChangeNodeTransform(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Change Node Transform".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["change-node-transform".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<ChangeNodeTransformMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-node-transform");
    }
}
//#endregion 🧪️Tests
