//! 🔺️ Exact create-scene insertion delta with exhaustive pre-state protection.

use crate::artifacts::gltf::schema::mutations::create_scene::private::{default_after, default_scene, insert_empty_scene, insertion_position, reject, scene_count, GltfCreateSceneRejection};
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

pub const ID: &str = "s.stdio.gltf.mutation.create-scene.v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfCreateSceneDiffPhase {
    Diff,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfCreateSceneDiff {
    pub id: String,
    pub version: u32,
    pub phase: GltfCreateSceneDiffPhase,
    pub touched_paths: Vec<String>,
    pub position: u32,
    pub expected_scene_count: u32,
    pub expected_default_scene_before: Option<u32>,
    pub expected_scenes_before: Vec<GltfScene>,
    pub scene: GltfScene,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn paths(position: u32, default_scene_before: Option<u32>) -> Result<Vec<String>, GltfCreateSceneRejection> {
    Ok(if default_scene_before == default_after(default_scene_before, position)? { vec![format!("document/scenes/{position}")] } else { vec![format!("document/scenes/{position}"), "document/scene".into()] })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn touched_paths(diff: &GltfCreateSceneDiff, _base: &GltfSnapshot) -> Result<Vec<String>, GltfCreateSceneRejection> {
    paths(diff.position, diff.expected_default_scene_before)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(diff: &GltfCreateSceneDiff, base: &GltfSnapshot) -> Result<(), GltfCreateSceneRejection> {
    if diff.id != ID || diff.version != 1 || diff.phase != GltfCreateSceneDiffPhase::Diff {
        return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "canonical identity or phase does not match"));
    }
    insertion_position(diff.position, base)?;
    if diff.expected_scene_count != scene_count(&base.document.scenes)? {
        return Err(reject("gltf.mutation.stale-diff", "diff/expectedSceneCount", "scene collection no longer matches the planned pre-state"));
    }
    if diff.expected_default_scene_before != default_scene(base)? {
        return Err(reject("gltf.mutation.stale-diff", "document/scene", "default scene no longer matches the planned pre-state"));
    }
    if diff.expected_scenes_before != base.document.scenes {
        return Err(reject("gltf.mutation.stale-diff", "document/scenes", "scene sequence no longer matches the planned pre-state"));
    }
    if diff.touched_paths != touched_paths(diff, base)? {
        return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must name every concrete changed location"));
    }
    if diff.scene != GltfScene::default() {
        return Err(reject("gltf.mutation.invalid-created-scene", "diff/scene", "create-scene may only insert the canonical empty scene"));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(diff: &GltfCreateSceneDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfCreateSceneRejection> {
    validate(diff, base)?;
    let mut next = base.clone();
    insert_empty_scene(&mut next, insertion_position(diff.position, base)?)?;
    Ok(next)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfCreateSceneDiff) -> Result<Vec<u8>, GltfCreateSceneRejection> {
    serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(base: &GltfSnapshot, position: u32) -> Result<GltfCreateSceneDiff, GltfCreateSceneRejection> {
    insertion_position(position, base)?;
    let mut diff = GltfCreateSceneDiff {
        id: ID.into(),
        version: 1,
        phase: GltfCreateSceneDiffPhase::Diff,
        touched_paths: Vec::new(),
        position,
        expected_scene_count: scene_count(&base.document.scenes)?,
        expected_default_scene_before: default_scene(base)?,
        expected_scenes_before: base.document.scenes.clone(),
        scene: GltfScene::default(),
    };
    diff.touched_paths = touched_paths(&diff, base)?;
    Ok(diff)
}
