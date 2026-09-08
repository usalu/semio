//! 🧬️ Direct move-primitive-attribute mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::modules::mutation_support::structure_geometry::checked_index;
pub const ID: &str = "s.stdio.gltf.mutation.move-primitive-attribute.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfMovePrimitiveAttributePayload { pub mesh: usize, pub primitive: usize, pub semantic: String, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMovePrimitiveAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; let attributes = &base.document.meshes[payload.mesh].primitives[payload.primitive].attributes; let index = attributes.iter().position(|(semantic, _)| semantic == &payload.semantic).ok_or_else(|| reject("gltf.mutation.relation-absent", "document/meshes/primitives/attributes", "semantic is not bound"))?; checked_index(payload.position, attributes.len(), "document/meshes/primitives/attributes")?; if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives/attributes", "destination equals source")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMovePrimitiveAttributePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let attributes = &mut next.document.meshes[payload.mesh].primitives[payload.primitive].attributes; let index = attributes.iter().position(|(semantic, _)| semantic == &payload.semantic).expect("validated semantic"); let attribute = attributes.remove(index); attributes.insert(payload.position, attribute); Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MovePrimitiveAttributeMutation {
    Apply(GltfMovePrimitiveAttributePayload),
    Restore(Box<crate::artifacts::gltf::schema::diff::GltfDiff>),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MovePrimitiveAttributeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "primitive-attribute", kind: "move-primitive-attribute", record: "MovedPrimitiveAttribute" };

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
        vec![super::GltfMutation::MovePrimitiveAttribute(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Move Primitive Attribute".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["move-primitive-attribute".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<MovePrimitiveAttributeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-primitive-attribute");
    }
}
//#endregion 🧪️Tests
