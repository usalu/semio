//! 🧬️ Direct change-material-double-sided mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::material_animation::{index, GltfMaterialAnimationFailure};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-material-double-sided.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials/{material}/doubleSided"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn touched_paths(payload: &GltfChangeMaterialDoubleSidedPayload) -> Vec<String> {
    vec![format!("document/materials/{}/doubleSided", payload.material)]
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeMaterialDoubleSidedRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn failure(value: GltfMaterialAnimationFailure) -> GltfChangeMaterialDoubleSidedRejection {
    GltfChangeMaterialDoubleSidedRejection { code: value.code.into(), path: value.path, detail: value.detail.into() }
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeMaterialDoubleSidedPayload {
    pub material: usize,
    pub double_sided: bool,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeMaterialDoubleSidedPayload, base: &GltfSnapshot) -> Result<(), GltfChangeMaterialDoubleSidedRejection> {
    index(&base.document.materials, payload.material, "document/materials").map_err(failure)?;
    (base.document.materials[payload.material].double_sided != payload.double_sided).then_some(()).ok_or_else(|| GltfChangeMaterialDoubleSidedRejection {
        code: "gltf.mutation.no-observable-change".into(),
        path: format!("document/materials/{}/doubleSided", payload.material),
        detail: "doubleSided already has that value".into(),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeMaterialDoubleSidedPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfChangeMaterialDoubleSidedRejection> {
    validate(p, base)?;
    let current = base.document.materials[p.material].double_sided;
    Ok(GltfDiff { materials: patch(p.material, GltfMaterialDiff { double_sided: (current != p.double_sided).then_some(p.double_sided), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeMaterialDoubleSidedPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = base.document.materials[p.material].double_sided;
    if current == p.double_sided {
        return Vec::new();
    }
    vec![super::change_material_double_sided::mutation(super::change_material_double_sided::GltfChangeMaterialDoubleSidedPayload { material: p.material, double_sided: current })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeMaterialDoubleSidedMutation {
    Apply(GltfChangeMaterialDoubleSidedPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeMaterialDoubleSidedPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeMaterialDoubleSided(ChangeMaterialDoubleSidedMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeMaterialDoubleSidedMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-double-sided", kind: "change-material-double-sided", record: "ChangedMaterialDoubleSided" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Material Double Sided", "Doppelseitigkeit des Materials ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-material-double-sided".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🪞️makes-the-6261b2/🦀️.rs"]
mod case_makes_the_6261b2;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
