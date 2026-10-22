//! 🧬️ Direct change-material-double-sided mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::material_animation::{index, GltfMaterialAnimationFailure};
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-material-double-sided.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials/{material}/doubleSided"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn touched_paths(payload: &GltfChangeMaterialDoubleSidedPayload) -> Vec<String> {
    vec![format!("document/materials/{}/doubleSided", payload.material)]
}
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
pub fn inverse<Op>(p: &GltfChangeMaterialDoubleSidedPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = base.document.materials[p.material].double_sided;
    if current == p.double_sided {
        return Vec::new();
    }
    vec![mutation(GltfChangeMaterialDoubleSidedPayload { material: p.material, double_sided: current })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeMaterialDoubleSidedMutation {
    Apply(GltfChangeMaterialDoubleSidedPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfChangeMaterialDoubleSidedPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::ChangeMaterialDoubleSidedMutation>,
{
    Op::from(ChangeMaterialDoubleSidedMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ChangeMaterialDoubleSidedMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ChangeMaterialDoubleSidedMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-double-sided", kind: "change-material-double-sided", record: "ChangedMaterialDoubleSided" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Material Double Sided", "Doppelseitigkeit des Materials ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-material-double-sided".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ChangeMaterialDoubleSidedMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ChangeMaterialDoubleSidedMutation>,
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


