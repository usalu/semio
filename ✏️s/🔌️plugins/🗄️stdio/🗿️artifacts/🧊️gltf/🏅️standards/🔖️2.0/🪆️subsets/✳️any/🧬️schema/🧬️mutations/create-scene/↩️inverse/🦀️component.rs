//! ↩️ Exact create-scene removal inverse with exhaustive post-state restoration.

use crate::artifacts::gltf::schema::mutations::create_scene::private::{default_after, default_scene, existing_position, reject, remove_created_scene, scene_count, GltfCreateSceneRejection};
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

pub const ID: &str = "s.stdio.gltf.mutation.create-scene.v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfCreateSceneInversePhase {
    Inverse,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfCreateSceneInverse {
    pub id: String,
    pub version: u32,
    pub phase: GltfCreateSceneInversePhase,
    pub touched_paths: Vec<String>,
    pub position: u32,
    pub expected_scene_count_after: u32,
    pub expected_scene: GltfScene,
    pub expected_scenes_after: Vec<GltfScene>,
    pub default_scene_before: Option<u32>,
    pub expected_default_scene_after: Option<u32>,
}

fn paths(position: u32, before: Option<u32>, after: Option<u32>) -> Vec<String> {
    if before == after {
        vec![format!("document/scenes/{}", position)]
    } else {
        vec![format!("document/scenes/{}", position), "document/scene".into()]
    }
}

fn validate_restored_default(inverse: &GltfCreateSceneInverse, after: &GltfSnapshot) -> Result<(), GltfCreateSceneRejection> {
    let restored_count = after.document.scenes.len() - 1;
    if inverse.default_scene_before.is_some_and(|scene| usize::try_from(scene).map_or(true, |scene| scene >= restored_count)) {
        return Err(reject("gltf.mutation.reference-out-of-range", "inverse/defaultSceneBefore", "restored default scene must name a surviving scene"));
    }
    if inverse.expected_default_scene_after != default_after(inverse.default_scene_before, inverse.position)? {
        return Err(reject("gltf.mutation.invalid-inverse-reference", "inverse/expectedDefaultSceneAfter", "default-scene repair must match the restored default scene"));
    }
    Ok(())
}

pub fn touched_paths(inverse: &GltfCreateSceneInverse, _after: &GltfSnapshot) -> Result<Vec<String>, GltfCreateSceneRejection> {
    Ok(paths(inverse.position, inverse.default_scene_before, inverse.expected_default_scene_after))
}

pub fn validate(inverse: &GltfCreateSceneInverse, after: &GltfSnapshot) -> Result<(), GltfCreateSceneRejection> {
    if inverse.id != ID || inverse.version != 1 || inverse.phase != GltfCreateSceneInversePhase::Inverse {
        return Err(reject("gltf.mutation.invalid-inverse-envelope", "inverse", "canonical identity or phase does not match"));
    }
    let position = existing_position(inverse.position, after)?;
    if inverse.expected_scene_count_after != scene_count(&after.document.scenes)? {
        return Err(reject("gltf.mutation.stale-inverse", "inverse/expectedSceneCountAfter", "scene collection no longer matches the forward-created state"));
    }
    validate_restored_default(inverse, after)?;
    if inverse.touched_paths != touched_paths(inverse, after)? {
        return Err(reject("gltf.mutation.invalid-touched-paths", "inverse/touchedPaths", "paths must name every concrete changed location"));
    }
    if inverse.expected_scene != GltfScene::default() {
        return Err(reject("gltf.mutation.invalid-created-scene", "inverse/expectedScene", "inverse must target the canonical empty scene"));
    }
    if inverse.expected_default_scene_after != default_scene(after)? {
        return Err(reject("gltf.mutation.stale-inverse", "document/scene", "default scene does not match the forward-created state"));
    }
    if after.document.scenes[position] != inverse.expected_scene {
        return Err(reject("gltf.mutation.stale-inverse", format!("document/scenes/{}", inverse.position), "current scene does not match the forward-created scene"));
    }
    if inverse.expected_scenes_after != after.document.scenes {
        return Err(reject("gltf.mutation.stale-inverse", "document/scenes", "scene sequence no longer matches the forward-created state"));
    }
    Ok(())
}

pub fn apply(inverse: &GltfCreateSceneInverse, after: &GltfSnapshot) -> Result<GltfSnapshot, GltfCreateSceneRejection> {
    validate(inverse, after)?;
    let mut next = after.clone();
    remove_created_scene(&mut next, existing_position(inverse.position, after)?, inverse.default_scene_before)?;
    Ok(next)
}

pub fn encode(inverse: &GltfCreateSceneInverse) -> Result<Vec<u8>, GltfCreateSceneRejection> {
    serde_json::to_vec(inverse).map_err(|error| reject("gltf.mutation.encode-failed", "inverse", error.to_string()))
}

pub fn derive(base: &GltfSnapshot, position: u32) -> Result<GltfCreateSceneInverse, GltfCreateSceneRejection> {
    let position_index = crate::artifacts::gltf::schema::mutations::create_scene::private::insertion_position(position, base)?;
    let default_scene_before = default_scene(base)?;
    let expected_default_scene_after = default_after(default_scene_before, position)?;
    let mut expected_scenes_after = base.document.scenes.clone();
    expected_scenes_after.insert(position_index, GltfScene::default());
    let mut inverse = GltfCreateSceneInverse {
        id: ID.into(),
        version: 1,
        phase: GltfCreateSceneInversePhase::Inverse,
        touched_paths: Vec::new(),
        position,
        expected_scene_count_after: scene_count(&expected_scenes_after)?,
        expected_scene: GltfScene::default(),
        expected_scenes_after,
        default_scene_before,
        expected_default_scene_after,
    };
    inverse.touched_paths = touched_paths(&inverse, base)?;
    Ok(inverse)
}
