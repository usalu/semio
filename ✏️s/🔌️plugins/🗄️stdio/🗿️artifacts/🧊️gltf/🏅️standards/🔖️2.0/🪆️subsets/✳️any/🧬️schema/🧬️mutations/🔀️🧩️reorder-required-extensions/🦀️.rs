//! 🧬️ Direct reorder-required-extensions mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-required-extensions.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderRequiredExtensionsPayload { pub order: Vec<String> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderRequiredExtensionsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.extensions_required.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>() .len() != payload.order.len() || payload.order.iter().any(|value| !base.document.extensions_required.contains(value)) { return Err(reject("gltf.mutation.invalid-permutation", "document/extensionsRequired", "order must contain every declaration exactly once")); } if payload.order == base.document.extensions_required { return Err(reject("gltf.mutation.no-observable-change", "document/extensionsRequired", "order already matches")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfReorderRequiredExtensionsPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions_required = payload.order.clone(); Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderRequiredExtensionsMutation {
    Apply(GltfReorderRequiredExtensionsPayload),
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

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderRequiredExtensionsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "required-extensions", kind: "reorder-required-extensions", record: "ReorderedRequiredExtensions" };

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
        vec![super::GltfMutation::ReorderRequiredExtensions(Self::Restore(inverse))]
    }

    fn label(&self) -> String {
        "Reorder Required Extensions".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-required-extensions".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<ReorderRequiredExtensionsMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-required-extensions");
    }
}
//#endregion 🧪️Tests
