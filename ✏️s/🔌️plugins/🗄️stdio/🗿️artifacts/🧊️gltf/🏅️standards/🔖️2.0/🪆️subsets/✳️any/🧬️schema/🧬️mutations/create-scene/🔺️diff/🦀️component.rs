//! 🔺️ Exact create-scene insertion delta with forward stale-state protection.
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
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
#[serde(rename_all = "camelCase")]
pub struct GltfCreateSceneDiff {
    pub id: String,
    pub version: u32,
    pub phase: GltfCreateSceneDiffPhase,
    pub touched_paths: Vec<String>,
    pub position: usize,
    pub expected_scene_count: usize,
    pub expected_default_scene_before: Option<usize>,
    pub expected_next_scene: Option<GltfScene>,
    pub scene: GltfScene,
}
fn default_after(default_scene: Option<usize>, position: usize) -> Result<Option<usize>, GltfTopLevelMutationRejection> {
    default_scene.map(|scene| if scene >= position { scene.checked_add(1).ok_or_else(|| reject("gltf.mutation.reference-overflow", "document/scene", "default scene cannot be remapped beyond usize")) } else { Ok(scene) }).transpose()
}
fn paths(base: &GltfSnapshot, position: usize) -> Result<Vec<String>, GltfTopLevelMutationRejection> {
    Ok(if base.document.scene == default_after(base.document.scene, position)? { vec![format!("document/scenes/{}", position)] } else { vec![format!("document/scenes/{}", position), "document/scene".into()] })
}
pub fn validate(diff: &GltfCreateSceneDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if diff.id != ID || diff.version != 1 || diff.phase != GltfCreateSceneDiffPhase::Diff {
        return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "canonical identity or phase does not match"));
    }
    if diff.position > base.document.scenes.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection"));
    }
    if diff.expected_scene_count != base.document.scenes.len() {
        return Err(reject("gltf.mutation.stale-diff", "diff/expectedSceneCount", "scene collection no longer matches the planned pre-state"));
    }
    if diff.expected_default_scene_before != base.document.scene {
        return Err(reject("gltf.mutation.stale-diff", "document/scene", "default scene no longer matches the planned pre-state"));
    }
    if diff.expected_next_scene != base.document.scenes.get(diff.position).cloned() {
        return Err(reject("gltf.mutation.stale-diff", format!("document/scenes/{}", diff.position), "insertion anchor no longer matches the planned pre-state"));
    }
    if diff.touched_paths != paths(base, diff.position)? {
        return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must name every concrete changed location"));
    }
    if diff.scene != GltfScene::default() {
        return Err(reject("gltf.mutation.invalid-created-scene", "diff/scene", "create-scene may only insert the canonical empty scene"));
    }
    Ok(())
}
pub fn apply(diff: &GltfCreateSceneDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(diff, base)?;
    let mut next = base.clone();
    repair(&mut next.document, GltfTopLevelFamily::Scenes, &Change::Insert(diff.position))?;
    next.document.scenes.insert(diff.position, diff.scene.clone());
    Ok(next)
}
pub fn encode(diff: &GltfCreateSceneDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> {
    serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string()))
}
pub fn derive(base: &GltfSnapshot, position: usize) -> Result<GltfCreateSceneDiff, GltfTopLevelMutationRejection> {
    if position > base.document.scenes.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection"));
    }
    Ok(GltfCreateSceneDiff {
        id: ID.into(),
        version: 1,
        phase: GltfCreateSceneDiffPhase::Diff,
        touched_paths: paths(base, position)?,
        position,
        expected_scene_count: base.document.scenes.len(),
        expected_default_scene_before: base.document.scene,
        expected_next_scene: base.document.scenes.get(position).cloned(),
        scene: GltfScene::default(),
    })
}
