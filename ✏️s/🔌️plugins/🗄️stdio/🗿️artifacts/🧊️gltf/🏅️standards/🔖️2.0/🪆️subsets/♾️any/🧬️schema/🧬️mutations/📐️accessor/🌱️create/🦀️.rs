//! 🧬️ Direct create-accessor mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::engine::{GltfAccessorType, GltfComponentType};
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
use crate::schema::modules::mutation_support::top_level_collections::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-accessor.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/accessors"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)] #[value(rename_all = "camelCase")]
pub struct GltfCreateAccessorPayload { pub position: usize, pub component_type: GltfComponentType, pub count: usize, pub kind: GltfAccessorType }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateAccessorPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.accessors.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/accessors", "position must be within the collection")); }   Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfCreateAccessorPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Accessors, &Change::Insert(payload.position))?; next.document.accessors.insert(payload.position, GltfAccessor { buffer_view: None, byte_offset: 0, component_type: payload.component_type, normalized: false, count: payload.count, kind: payload.kind, max: None, min: None, sparse: None, name: None, extensions: None, extras: None }); Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateAccessorMutation {
    Apply(GltfCreateAccessorPayload),
    Restore(Box<crate::schema::diff::GltfDiff>),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for CreateAccessorMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "accessor", kind: "create-accessor", record: "CreatedAccessor" };

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
        vec![super::GltfMutation::CreateAccessor(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Create Accessor".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["create-accessor".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<CreateAccessorMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-accessor");
    }
}
//#endregion 🧪️Tests
