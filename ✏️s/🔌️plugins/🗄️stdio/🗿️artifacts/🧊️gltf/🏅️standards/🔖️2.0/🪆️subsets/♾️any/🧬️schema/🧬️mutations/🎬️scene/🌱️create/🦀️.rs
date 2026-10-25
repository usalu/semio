//! 🧬️ Direct create-scene mutation owner: payload, validation, typed diff, inverse, and outcomes.

use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::create_scene::{insertion_position, GltfCreateSceneRejection};
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;

pub const ID: &str = "s.stdio.gltf.mutation.create-scene.v1";

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfCreateScenePayload {
    pub position: u32,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<Box<GltfScene>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateScenePayload, base: &GltfSnapshot) -> Result<(), GltfCreateSceneRejection> {
    insertion_position(payload.position, base)?;
    if let Some(record) = &payload.scene {
        check_scene(record, &GltfLengths::after_insert(base, GltfTopLevelFamily::Scenes)).map_err(|error| GltfCreateSceneRejection { code: error.code, path: error.path, detail: error.detail })?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreateScenePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfCreateSceneRejection> {
    validate(p, base)?;
    let position = insertion_position(p.position, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Scenes, &mut after_insert(position));
    diff.scenes.get_or_insert_with(Default::default).added.push(GltfAdded { index: position, item: p.scene.as_deref().cloned().unwrap_or_default() });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfCreateScenePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_scene::DeleteSceneMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Ok(position) = insertion_position(p.position, base) else {
        return Vec::new();
    };
    vec![crate::standards::v2_0::subsets::any::schema::mutations::delete_scene::mutation(crate::standards::v2_0::subsets::any::schema::mutations::delete_scene::GltfDeleteScenePayload { index: position })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateSceneMutation {
    Apply(GltfCreateScenePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfCreateScenePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::create_scene::CreateSceneMutation>,
{
    Op::from(CreateSceneMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for CreateSceneMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<CreateSceneMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "scene", kind: "create-scene", record: "CreatedScene" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Create Scene", "Szene erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-scene".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for CreateSceneMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<CreateSceneMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::delete_scene::DeleteSceneMutation>,
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


