//! 🧬️ Direct change-asset-version mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-version.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/version"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeAssetVersionPayload {
    pub version: String,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeAssetVersionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.version.trim().is_empty() {
        return Err(reject("gltf.mutation.invalid-asset-version", "document/asset/version", "version must be non-empty"));
    }
    if payload.version == base.document.asset.version {
        return Err(reject("gltf.mutation.no-observable-change", "document/asset/version", "version already has this value"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeAssetVersionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { asset: changed(GltfAssetDiff { version: (base.document.asset.version != p.version).then(|| p.version.clone()), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfChangeAssetVersionPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    if base.document.asset.version == p.version {
        return Vec::new();
    }
    vec![mutation(GltfChangeAssetVersionPayload { version: base.document.asset.version.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeAssetVersionMutation {
    Apply(GltfChangeAssetVersionPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfChangeAssetVersionPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::change_asset_version::ChangeAssetVersionMutation>,
{
    Op::from(ChangeAssetVersionMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ChangeAssetVersionMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ChangeAssetVersionMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "asset-version", kind: "change-asset-version", record: "ChangedAssetVersion" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Asset Version", "Asset-Version ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-asset-version".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ChangeAssetVersionMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ChangeAssetVersionMutation>,
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
