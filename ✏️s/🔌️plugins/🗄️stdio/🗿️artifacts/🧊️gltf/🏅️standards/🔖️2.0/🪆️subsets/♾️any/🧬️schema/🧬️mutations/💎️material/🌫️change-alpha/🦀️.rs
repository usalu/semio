//! 🧬️ Direct change-material-alpha-mode mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::material_animation::{index, GltfMaterialAnimationFailure};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::snapshot::GltfAlphaMode;
use crate::GltfSnapshot;

pub const ID: &str = "s.stdio.gltf.mutation.change-material-alpha-mode.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials/{material}/alphaMode"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn touched_paths(payload: &GltfChangeMaterialAlphaModePayload) -> Vec<String> {
    vec![format!("document/materials/{}/alphaMode", payload.material)]
}
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeMaterialAlphaModeRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn failure(value: GltfMaterialAnimationFailure) -> GltfChangeMaterialAlphaModeRejection {
    GltfChangeMaterialAlphaModeRejection { code: value.code.into(), path: value.path, detail: value.detail.into() }
}
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeMaterialAlphaModePayload {
    pub material: usize,
    pub alpha_mode: GltfAlphaMode,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeMaterialAlphaModePayload, base: &GltfSnapshot) -> Result<(), GltfChangeMaterialAlphaModeRejection> {
    index(&base.document.materials, payload.material, "document/materials").map_err(failure)?;
    (base.document.materials[payload.material].alpha_mode != payload.alpha_mode).then_some(()).ok_or_else(|| GltfChangeMaterialAlphaModeRejection {
        code: "gltf.mutation.no-observable-change".into(),
        path: format!("document/materials/{}/alphaMode", payload.material),
        detail: "alphaMode already has that value".into(),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeMaterialAlphaModePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfChangeMaterialAlphaModeRejection> {
    validate(p, base)?;
    let current = base.document.materials[p.material].alpha_mode;
    Ok(GltfDiff { materials: patch(p.material, GltfMaterialDiff { alpha_mode: (current != p.alpha_mode).then_some(p.alpha_mode), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeMaterialAlphaModePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = base.document.materials[p.material].alpha_mode;
    if current == p.alpha_mode {
        return Vec::new();
    }
    vec![super::change_material_alpha_mode::mutation(super::change_material_alpha_mode::GltfChangeMaterialAlphaModePayload { material: p.material, alpha_mode: current })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeMaterialAlphaModeMutation {
    Apply(GltfChangeMaterialAlphaModePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeMaterialAlphaModePayload) -> super::GltfMutation {
    super::GltfMutation::ChangeMaterialAlphaMode(ChangeMaterialAlphaModeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeMaterialAlphaModeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-alpha-mode", kind: "change-material-alpha-mode", record: "ChangedMaterialAlphaMode" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Material Alpha Mode", "Alpha-Modus des Materials ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-material-alpha-mode".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🎭️switches-the-79b834/🦀️.rs"]
mod case_switches_the_79b834;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
