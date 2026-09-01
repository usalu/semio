//! 🧬️ Direct reorder-morph-targets mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-morph-targets.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderMorphTargetsPayload { pub mesh: usize, pub primitive: usize, pub order: Vec<usize> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderMorphTargetsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; let length = base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(); if payload.order.len() != length || payload.order.iter().any(|index| *index >= length) || { let mut order = payload.order.clone(); order.sort_unstable(); order.dedup(); order.len() != length } { return Err(reject("gltf.mutation.invalid-permutation", "document/meshes/primitives/targets", "order must contain each target once")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfReorderMorphTargetsPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let prior = next.document.meshes[payload.mesh].primitives[payload.primitive].targets.clone(); next.document.meshes[payload.mesh].primitives[payload.primitive].targets = payload.order.iter().map(|index| prior[*index].clone()).collect(); Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderMorphTargetsMutation {
    Apply(GltfReorderMorphTargetsPayload),
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

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderMorphTargetsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "morph-targets", kind: "reorder-morph-targets", record: "ReorderedMorphTargets" };

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
        vec![super::GltfMutation::ReorderMorphTargets(Self::Restore(inverse))]
    }

    fn label(&self) -> String {
        "Reorder Morph Targets".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-morph-targets".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<ReorderMorphTargetsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-morph-targets");
    }
}
//#endregion 🧪️Tests
