//! 🧬️ Direct move-used-extension mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-used-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsUsed"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveUsedExtensionPayload {
    pub extension: String,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveUsedExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    let index = base.document.extensions_used.iter().position(|value| value == &payload.extension).ok_or_else(|| reject("gltf.mutation.extension-absent", "document/extensionsUsed", "extension is not declared"))?;
    if payload.position >= base.document.extensions_used.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/extensionsUsed", "position must address a declaration"));
    }
    if index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/extensionsUsed", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMoveUsedExtensionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let Some(index) = base.document.extensions_used.iter().position(|value| value == &p.extension) else {
        return Err(reject("gltf.mutation.extension-absent", "document/extensionsUsed", "extension is not declared"));
    };
    Ok(GltfDiff { extensions_used: Some(GltfStringsDelta::relocation(&base.document.extensions_used, index, p.position)), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfMoveUsedExtensionPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.extensions_used.iter().position(|value| value == &p.extension) else {
        return Vec::new();
    };
    vec![super::move_used_extension::mutation(super::move_used_extension::GltfMoveUsedExtensionPayload { extension: p.extension.clone(), position: index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveUsedExtensionMutation {
    Apply(GltfMoveUsedExtensionPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfMoveUsedExtensionPayload) -> super::GltfMutation {
    super::GltfMutation::MoveUsedExtension(MoveUsedExtensionMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveUsedExtensionMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "used-extension", kind: "move-used-extension", record: "MovedUsedExtension" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Move Used Extension", "Verwendete Erweiterung verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-used-extension".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t058/🦀️.rs"]
mod case_t058;
//#endregion 🧪️Tests
