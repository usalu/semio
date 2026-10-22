//! 🧬️ Direct add-used-extension mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
#[cfg(test)] use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.add-used-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsUsed"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfDeclareUsedExtensionPayload {
    pub extension: String,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfDeclareUsedExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.extension.trim().is_empty() {
        return Err(reject("gltf.mutation.invalid-extension", "document/extensionsUsed", "extension must be non-empty"));
    }
    if base.document.extensions_used.contains(&payload.extension) {
        return Err(reject("gltf.mutation.duplicate-extension", "document/extensionsUsed", "extension is already declared"));
    }
    if payload.position > base.document.extensions_used.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/extensionsUsed", "position must be within the declaration list"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfDeclareUsedExtensionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { extensions_used: Some(GltfStringsDelta::insertion(p.position, p.extension.clone())), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfDeclareUsedExtensionPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::mutation(crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::GltfWithdrawUsedExtensionPayload { extension: p.extension.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum AddUsedExtensionMutation {
    Apply(GltfDeclareUsedExtensionPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfDeclareUsedExtensionPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::add_used_extension::AddUsedExtensionMutation>,
{
    Op::from(AddUsedExtensionMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for AddUsedExtensionMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<AddUsedExtensionMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "used-extension", kind: "add-used-extension", record: "AddedUsedExtension" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Add Used Extension", "Verwendete Erweiterung hinzufügen")
    }

    fn target(&self) -> Vec<String> {
        vec!["add-used-extension".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for AddUsedExtensionMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<AddUsedExtensionMutation>,
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
