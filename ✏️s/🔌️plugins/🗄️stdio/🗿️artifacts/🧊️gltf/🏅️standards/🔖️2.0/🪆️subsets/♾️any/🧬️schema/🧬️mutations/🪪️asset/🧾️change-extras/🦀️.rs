//! 🧬️ Direct change-asset-extra-data mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfJson;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-extra-data.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/extras"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
pub fn inverse<Op>(p: &GltfChangeAssetExtraDataPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.asset.extras;
    if current == &p.data {
        return Vec::new();
    }
    vec![mutation(GltfChangeAssetExtraDataPayload { data: current.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeAssetExtraDataMutation {
    Apply(GltfChangeAssetExtraDataPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfChangeAssetExtraDataPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::change_asset_extra_data::ChangeAssetExtraDataMutation>,
{
    Op::from(ChangeAssetExtraDataMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ChangeAssetExtraDataMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ChangeAssetExtraDataMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "asset-extra-data", kind: "change-asset-extra-data", record: "ChangedAssetExtraData" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Asset Extra Data", "Zusatzdaten des Assets ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-asset-extra-data".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ChangeAssetExtraDataMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ChangeAssetExtraDataMutation>,
{
    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }
    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests

//#endregion 🧪️Tests
