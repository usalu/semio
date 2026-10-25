//! 🧬️ Direct change-material-alpha-mode mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::material_animation::{index, GltfMaterialAnimationFailure};
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAlphaMode;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;

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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
pub fn inverse<Op>(p: &GltfChangeMaterialAlphaModePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::ChangeMaterialAlphaModeMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = base.document.materials[p.material].alpha_mode;
    if current == p.alpha_mode {
        return Vec::new();
    }
    vec![mutation(GltfChangeMaterialAlphaModePayload { material: p.material, alpha_mode: current })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeMaterialAlphaModeMutation {
    Apply(GltfChangeMaterialAlphaModePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfChangeMaterialAlphaModePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::ChangeMaterialAlphaModeMutation>,
{
    Op::from(ChangeMaterialAlphaModeMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ChangeMaterialAlphaModeMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ChangeMaterialAlphaModeMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-alpha-mode", kind: "change-material-alpha-mode", record: "ChangedMaterialAlphaMode" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Material Alpha Mode", "Alpha-Modus des Materials ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-material-alpha-mode".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ChangeMaterialAlphaModeMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ChangeMaterialAlphaModeMutation>,
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


