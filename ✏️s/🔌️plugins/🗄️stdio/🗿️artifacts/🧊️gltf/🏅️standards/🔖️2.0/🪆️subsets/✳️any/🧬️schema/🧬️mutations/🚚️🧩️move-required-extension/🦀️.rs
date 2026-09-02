//! 🧬️ Direct move-required-extension mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.move-required-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveRequiredExtensionPayload { pub extension: String, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveRequiredExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { let index = base.document.extensions_required.iter().position(|value| value == &payload.extension).ok_or_else(|| reject("gltf.mutation.extension-absent", "document/extensionsRequired", "extension is not declared"))?; if payload.position >= base.document.extensions_required.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/extensionsRequired", "position must address a declaration")); } if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/extensionsRequired", "destination equals source")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveRequiredExtensionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let value = next.document.extensions_required.remove(next.document.extensions_required.iter().position(|value| value == &payload.extension).unwrap()); next.document.extensions_required.insert(payload.position, value); Ok(next) }

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveRequiredExtensionMutation {
    Apply(GltfMoveRequiredExtensionPayload),
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

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveRequiredExtensionMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "required-extension", kind: "move-required-extension", record: "MovedRequiredExtension" };

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
        vec![super::GltfMutation::MoveRequiredExtension(Self::Restore(inverse))]
    }

    fn label(&self) -> String {
        "Move Required Extension".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["move-required-extension".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<MoveRequiredExtensionMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "move-required-extension");
    }
}
//#endregion 🧪️Tests
