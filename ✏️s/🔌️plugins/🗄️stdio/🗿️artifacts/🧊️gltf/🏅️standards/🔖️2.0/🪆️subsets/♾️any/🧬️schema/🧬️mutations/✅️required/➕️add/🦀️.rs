//! 🧬️ Direct add-required-extension mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.add-required-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfRequireExtensionPayload {
    pub extension: String,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfRequireExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.extension.trim().is_empty() {
        return Err(reject("gltf.mutation.invalid-extension", "document/extensionsRequired", "extension must be non-empty"));
    }
    if base.document.extensions_required.contains(&payload.extension) {
        return Err(reject("gltf.mutation.duplicate-extension", "document/extensionsRequired", "extension is already declared"));
    }
    if !base.document.extensions_used.contains(&payload.extension) {
        return Err(reject("gltf.mutation.required-extension-not-used", "document/extensionsRequired", "a required extension must first be used"));
    }
    if payload.position > base.document.extensions_required.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/extensionsRequired", "position must be within the declaration list"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfRequireExtensionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { extensions_required: Some(GltfStringsDelta::insertion(p.position, p.extension.clone())), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfRequireExtensionPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::remove_required_extension::mutation(super::remove_required_extension::GltfUnrequireExtensionPayload { extension: p.extension.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum AddRequiredExtensionMutation {
    Apply(GltfRequireExtensionPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfRequireExtensionPayload) -> super::GltfMutation {
    super::GltfMutation::AddRequiredExtension(AddRequiredExtensionMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for AddRequiredExtensionMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "required-extension", kind: "add-required-extension", record: "AddedRequiredExtension" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Add Required Extension", "Erforderliche Erweiterung hinzufügen")
    }

    fn target(&self) -> Vec<String> {
        vec!["add-required-extension".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t041/🦀️.rs"]
mod case_t041;
//#endregion 🧪️Tests
