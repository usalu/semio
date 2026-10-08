//! 🧬️ Direct move-required-extension mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-required-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveRequiredExtensionPayload {
    pub extension: String,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveRequiredExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    let index = base.document.extensions_required.iter().position(|value| value == &payload.extension).ok_or_else(|| reject("gltf.mutation.extension-absent", "document/extensionsRequired", "extension is not declared"))?;
    if payload.position >= base.document.extensions_required.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/extensionsRequired", "position must address a declaration"));
    }
    if index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/extensionsRequired", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMoveRequiredExtensionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let Some(index) = base.document.extensions_required.iter().position(|value| value == &p.extension) else {
        return Err(reject("gltf.mutation.extension-absent", "document/extensionsRequired", "extension is not declared"));
    };
    Ok(GltfDiff { extensions_required: Some(with_moved(&base.document.extensions_required, index, p.position)), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfMoveRequiredExtensionPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.extensions_required.iter().position(|value| value == &p.extension) else {
        return Vec::new();
    };
    vec![super::move_required_extension::mutation(super::move_required_extension::GltfMoveRequiredExtensionPayload { extension: p.extension.clone(), position: index })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveRequiredExtensionMutation {
    Apply(GltfMoveRequiredExtensionPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfMoveRequiredExtensionPayload) -> super::GltfMutation {
    super::GltfMutation::MoveRequiredExtension(MoveRequiredExtensionMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveRequiredExtensionMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "required-extension", kind: "move-required-extension", record: "MovedRequiredExtension" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Move Required Extension", "Erforderliche Erweiterung verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-required-extension".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t043/🦀️.rs"]
mod case_t043;
//#endregion 🧪️Tests
