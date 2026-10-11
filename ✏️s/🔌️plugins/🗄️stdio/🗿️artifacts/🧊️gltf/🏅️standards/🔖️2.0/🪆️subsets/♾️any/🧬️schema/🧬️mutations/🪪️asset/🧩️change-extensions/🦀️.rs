//! 🧬️ Direct change-asset-extension-data mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::schema::snapshot::GltfJson;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-extension-data.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/extensions"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeAssetExtensionDataPayload {
    pub data: Option<GltfJson>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeAssetExtensionDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.data == base.document.asset.extensions {
        return Err(reject("gltf.mutation.no-observable-change", "document/asset/extensions", "value already has this value"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeAssetExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.asset.extensions;
    Ok(GltfDiff { asset: changed(GltfAssetDiff { extensions: (current != &p.data).then(|| p.data.clone()), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeAssetExtensionDataPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.asset.extensions;
    if current == &p.data {
        return Vec::new();
    }
    vec![super::change_asset_extension_data::mutation(super::change_asset_extension_data::GltfChangeAssetExtensionDataPayload { data: current.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeAssetExtensionDataMutation {
    Apply(GltfChangeAssetExtensionDataPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeAssetExtensionDataPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeAssetExtensionData(ChangeAssetExtensionDataMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeAssetExtensionDataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "asset-extension-data", kind: "change-asset-extension-data", record: "ChangedAssetExtensionData" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Asset Extension Data", "Erweiterungsdaten des Assets ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-asset-extension-data".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t069/🦀️.rs"]
mod case_t069;
//#endregion 🧪️Tests
