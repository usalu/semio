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

async fn paths(position: u32, default_scene_before: Option<u32>) -> Result<Vec<String>, GltfCreateSceneRejection> {
    Ok(if default_scene_before == default_after(default_scene_before, position).await? { vec![format!("document/scenes/{position}")] } else { vec![format!("document/scenes/{position}"), "document/scene".into()] })
}

pub async fn touched_paths(diff: &GltfCreateSceneDiff, _base: &GltfSnapshot) -> Result<Vec<String>, GltfCreateSceneRejection> {
    paths(diff.position, diff.expected_default_scene_before).await
}

pub async fn validate(diff: &GltfCreateSceneDiff, base: &GltfSnapshot) -> Result<(), GltfCreateSceneRejection> {
    if diff.id != ID || diff.version != 1 || diff.phase != GltfCreateSceneDiffPhase::Diff {
        return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "canonical identity or phase does not match").await);
    }
    insertion_position(diff.position, base).await?;
    if diff.expected_scene_count != scene_count(&base.document.scenes).await? {
        return Err(reject("gltf.mutation.stale-diff", "diff/expectedSceneCount", "scene collection no longer matches the planned pre-state").await);
    }
    if diff.expected_default_scene_before != default_scene(base).await? {
        return Err(reject("gltf.mutation.stale-diff", "document/scene", "default scene no longer matches the planned pre-state").await);
    }
    if diff.expected_scenes_before != base.document.scenes {
        return Err(reject("gltf.mutation.stale-diff", "document/scenes", "scene sequence no longer matches the planned pre-state").await);
    }
    if diff.touched_paths != touched_paths(diff, base).await? {
        return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must name every concrete changed location").await);
    }
    if diff.scene != GltfScene::default() {
        return Err(reject("gltf.mutation.invalid-created-scene", "diff/scene", "create-scene may only insert the canonical empty scene").await);
    }
    Ok(())
}

pub async fn apply(diff: &GltfCreateSceneDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfCreateSceneRejection> {
    validate(diff, base).await?;
    let mut next = base.clone();
    insert_empty_scene(&mut next, insertion_position(diff.position, base).await?).await?;
    Ok(next)
}

pub async fn encode(diff: &GltfCreateSceneDiff) -> Result<Vec<u8>, GltfCreateSceneRejection> {
    serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string()))
}

pub async fn derive(base: &GltfSnapshot, position: u32) -> Result<GltfCreateSceneDiff, GltfCreateSceneRejection> {
    insertion_position(position, base).await?;
    let mut diff = GltfCreateSceneDiff {
        id: ID.into(),
        version: 1,
        phase: GltfCreateSceneDiffPhase::Diff,
        touched_paths: Vec::new(),
        position,
        expected_scene_count: scene_count(&base.document.scenes).await?,
        expected_default_scene_before: default_scene(base).await?,
        expected_scenes_before: base.document.scenes.clone(),
        scene: GltfScene::default(),
    };
    diff.touched_paths = touched_paths(&diff, base).await?;
    Ok(diff)
}
