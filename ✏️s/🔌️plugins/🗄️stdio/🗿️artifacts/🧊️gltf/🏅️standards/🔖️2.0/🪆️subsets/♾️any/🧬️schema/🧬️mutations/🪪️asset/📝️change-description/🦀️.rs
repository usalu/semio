//! 🧬️ Direct change-asset-descriptive-metadata mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-descriptive-metadata.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/generator", "document/asset/copyright", "document/asset/minVersion"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
pub fn plan(p: &GltfChangeAssetDescriptiveMetadataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let asset = &base.document.asset;
    Ok(GltfDiff { asset: changed(GltfAssetDiff { generator: (asset.generator != p.generator).then(|| p.generator.clone()), copyright: (asset.copyright != p.copyright).then(|| p.copyright.clone()), min_version: (asset.min_version != p.min_version).then(|| p.min_version.clone()), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeAssetDescriptiveMetadataPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let asset = &base.document.asset;
    if asset.generator == p.generator && asset.copyright == p.copyright && asset.min_version == p.min_version {
        return Vec::new();
    }
    vec![super::change_asset_descriptive_metadata::mutation(super::change_asset_descriptive_metadata::GltfChangeAssetDescriptiveMetadataPayload { generator: asset.generator.clone(), copyright: asset.copyright.clone(), min_version: asset.min_version.clone() })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeAssetDescriptiveMetadataMutation {
    Apply(GltfChangeAssetDescriptiveMetadataPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeAssetDescriptiveMetadataPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeAssetDescriptiveMetadata(ChangeAssetDescriptiveMetadataMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeAssetDescriptiveMetadataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "asset-descriptive-metadata", kind: "change-asset-descriptive-metadata", record: "ChangedAssetDescriptiveMetadata" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<super::GltfMutation>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Asset Descriptive Metadata", "Beschreibende Asset-Metadaten ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-asset-descriptive-metadata".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t068/🦀️.rs"]
mod case_t068;
//#endregion 🧪️Tests
