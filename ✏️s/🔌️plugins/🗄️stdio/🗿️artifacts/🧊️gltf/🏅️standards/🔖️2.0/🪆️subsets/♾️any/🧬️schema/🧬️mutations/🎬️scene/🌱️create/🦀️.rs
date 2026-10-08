//! 🧬️ Direct create-scene mutation owner: payload, validation, typed diff, inverse, and outcomes.

use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::schema::modules::mutation_support::create_scene::{insert_empty_scene, insertion_position, GltfCreateSceneRejection};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::GltfSnapshot;

pub const ID: &str = "s.stdio.gltf.mutation.create-scene.v1";

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
pub fn inverse(p: &GltfCreateScenePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Ok(position) = insertion_position(p.position, base) else {
        return Vec::new();
    };
    vec![super::delete_scene::mutation(super::delete_scene::GltfDeleteScenePayload { index: position })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateSceneMutation {
    Apply(GltfCreateScenePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfCreateScenePayload) -> super::GltfMutation {
    super::GltfMutation::CreateScene(CreateSceneMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for CreateSceneMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "scene", kind: "create-scene", record: "CreatedScene" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Create Scene", "Szene erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-scene".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🎞️inserts-an-empty-3bc9ce/🦀️.rs"]
mod case_inserts_an_empty_3bc9ce;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
