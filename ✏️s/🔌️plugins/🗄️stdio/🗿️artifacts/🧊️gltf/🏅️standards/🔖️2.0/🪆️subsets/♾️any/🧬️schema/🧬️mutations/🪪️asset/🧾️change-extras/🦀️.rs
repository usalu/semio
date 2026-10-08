//! 🧬️ Direct change-asset-extra-data mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::schema::snapshot::GltfJson;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-extra-data.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/extras"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeAssetExtraDataPayload {
    pub data: Option<GltfJson>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeAssetExtraDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.data == base.document.asset.extras {
        return Err(reject("gltf.mutation.no-observable-change", "document/asset/extras", "value already has this value"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeAssetExtraDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.asset.extras;
    Ok(GltfDiff { asset: changed(GltfAssetDiff { extras: (current != &p.data).then(|| p.data.clone()), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeAssetExtraDataPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.asset.extras;
    if current == &p.data {
        return Vec::new();
    }
    vec![super::change_asset_extra_data::mutation(super::change_asset_extra_data::GltfChangeAssetExtraDataPayload { data: current.clone() })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeAssetExtraDataMutation {
    Apply(GltfChangeAssetExtraDataPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeAssetExtraDataPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeAssetExtraData(ChangeAssetExtraDataMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeAssetExtraDataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "asset-extra-data", kind: "change-asset-extra-data", record: "ChangedAssetExtraData" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Asset Extra Data", "Zusatzdaten des Assets ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-asset-extra-data".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t070/🦀️.rs"]
mod case_t070;
//#endregion 🧪️Tests
