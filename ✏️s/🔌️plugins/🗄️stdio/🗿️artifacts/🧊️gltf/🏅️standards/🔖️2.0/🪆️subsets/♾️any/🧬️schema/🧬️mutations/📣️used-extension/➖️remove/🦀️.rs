//! 🧬️ Direct remove-used-extension mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.remove-used-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsUsed"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfWithdrawUsedExtensionPayload { pub extension: String }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfWithdrawUsedExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if !base.document.extensions_used.contains(&payload.extension) { return Err(reject("gltf.mutation.extension-absent", "document/extensionsUsed", "extension is not declared")); }
    if base.document.extensions_required.contains(&payload.extension) { return Err(reject("gltf.mutation.extension-required", "document/extensionsRequired", "remove the requirement first")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfWithdrawUsedExtensionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions_used.retain(|value| value != &payload.extension); Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum RemoveUsedExtensionMutation {
    Apply(GltfWithdrawUsedExtensionPayload),
    Restore(Box<crate::artifacts::gltf::schema::diff::GltfDiff>),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for RemoveUsedExtensionMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "used-extension", kind: "remove-used-extension", record: "RemovedUsedExtension" };

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
        vec![super::GltfMutation::RemoveUsedExtension(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Remove Used Extension".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["remove-used-extension".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<RemoveUsedExtensionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "remove-used-extension");
    }
}
//#endregion 🧪️Tests
