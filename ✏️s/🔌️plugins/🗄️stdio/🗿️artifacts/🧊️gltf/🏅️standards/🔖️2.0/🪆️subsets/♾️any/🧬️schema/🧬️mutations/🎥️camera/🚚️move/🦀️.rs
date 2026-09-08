//! 🧬️ Direct move-camera mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::modules::mutation_support::top_level_collections::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-camera.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/cameras"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)] #[value(rename_all = "camelCase")]
pub struct GltfMoveCameraPayload { pub index: usize, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveCameraPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.cameras.len() || payload.position >= base.document.cameras.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/cameras", "indices must address items")); }
    if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/cameras", "destination equals source")); }  Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveCameraPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); cameras_op(&mut next, GltfTopLevelFamily::Cameras, payload.index, Some(payload.position), None)?;  Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveCameraMutation {
    Apply(GltfMoveCameraPayload),
    Restore(Box<crate::artifacts::gltf::schema::diff::GltfDiff>),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveCameraMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "camera", kind: "move-camera", record: "MovedCamera" };

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
        vec![super::GltfMutation::MoveCamera(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Move Camera".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["move-camera".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<MoveCameraMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-camera");
    }
}
//#endregion 🧪️Tests
