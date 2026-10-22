//! 🧬️ Direct change-scene-name mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::GltfTopLevelMutationRejection;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-scene-name.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeSceneNamePayload {
    pub scene: usize,
    pub value: Option<String>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeSceneNamePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeSceneNamePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.scenes[p.scene].name;
    Ok(GltfDiff { scenes: patch(p.scene, GltfSceneDiff { name: (current != &p.value).then(|| p.value.clone()), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfChangeSceneNamePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.scenes[p.scene].name;
    if current == &p.value {
        return Vec::new();
    }
    vec![mutation(GltfChangeSceneNamePayload { scene: p.scene, value: current.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeSceneNameMutation {
    Apply(GltfChangeSceneNamePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfChangeSceneNamePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::change_scene_name::ChangeSceneNameMutation>,
{
    Op::from(ChangeSceneNameMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for ChangeSceneNameMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<ChangeSceneNameMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "scene-name", kind: "change-scene-name", record: "ChangedSceneName" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Scene Name", "Szenenname ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-scene-name".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for ChangeSceneNameMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<ChangeSceneNameMutation>,
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
