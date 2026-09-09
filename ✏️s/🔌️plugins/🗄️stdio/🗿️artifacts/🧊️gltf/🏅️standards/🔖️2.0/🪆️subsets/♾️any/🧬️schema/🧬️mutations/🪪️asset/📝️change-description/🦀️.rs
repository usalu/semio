//! 🧬️ Direct change-asset-descriptive-metadata mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-descriptive-metadata.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/generator", "document/asset/copyright", "document/asset/minVersion"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeAssetDescriptiveMetadataPayload {
    pub generator: Option<String>,
    pub copyright: Option<String>,
    pub min_version: Option<String>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeAssetDescriptiveMetadataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.generator == base.document.asset.generator && payload.copyright == base.document.asset.copyright && payload.min_version == base.document.asset.min_version {
        return Err(reject("gltf.mutation.no-observable-change", "document/asset", "descriptive metadata already has these values"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfChangeAssetDescriptiveMetadataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let mut next = base.clone();
    next.document.asset.generator = payload.generator.clone();
    next.document.asset.copyright = payload.copyright.clone();
    next.document.asset.min_version = payload.min_version.clone();
    Ok(next)
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeAssetDescriptiveMetadataMutation {
    Apply(GltfChangeAssetDescriptiveMetadataPayload),
    Restore(Box<crate::schema::diff::GltfDiff>),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeAssetDescriptiveMetadataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "asset-descriptive-metadata", kind: "change-asset-descriptive-metadata", record: "ChangedAssetDescriptiveMetadata" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => match apply(payload, base) {
                Ok(next) => protocol::MutationOutcome::new(<crate::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
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
        vec![super::GltfMutation::ChangeAssetDescriptiveMetadata(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Change Asset Descriptive Metadata".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["change-asset-descriptive-metadata".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️direct-leaf/🦀️.rs"]
mod direct_leaf_tests;
//#endregion 🧪️Tests
