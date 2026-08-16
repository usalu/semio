//! 🔺️ Typed sparse deletion delta owned exclusively by delete-scene.
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, scenes_op, GltfTopLevelFamily, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.delete-scene.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfDeleteSceneDiffPhase {
    Diff,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfDeleteSceneDiff {
    pub id: String,
    pub version: u32,
    pub phase: GltfDeleteSceneDiffPhase,
    pub touched_paths: Vec<String>,
    pub index: usize,
    pub deleted_scene: GltfScene,
    pub default_scene_after: Option<usize>,
}
fn remap_default(scene: Option<usize>, index: usize) -> Option<usize> {
    match scene {
        None => None,
        Some(scene) if scene == index => None,
        Some(scene) if scene > index => Some(scene - 1),
        Some(scene) => Some(scene),
    }
}
fn paths(before: Option<usize>, after: Option<usize>, index: usize) -> Vec<String> {
    let mut paths = vec![format!("document/scenes/{}", index)];
    if before != after {
        paths.push("document/scene".into());
    }
    paths
}
pub fn validate(diff: &GltfDeleteSceneDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if diff.id != ID || diff.version != 1 || diff.phase != GltfDeleteSceneDiffPhase::Diff {
        return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "canonical identity or phase does not match"));
    }
    if diff.index >= base.document.scenes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "index must address a scene"));
    }
    if base.document.scene.is_some_and(|scene| scene >= base.document.scenes.len()) {
        return Err(reject("gltf.reference.invalid-default-scene", "document/scene", "default scene must address an existing scene"));
    }
    if diff.touched_paths != paths(base.document.scene, remap_default(base.document.scene, diff.index), diff.index) {
        return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete deletion effects"));
    }
    if diff.deleted_scene != base.document.scenes[diff.index] {
        return Err(reject("gltf.mutation.stale-diff", format!("document/scenes/{}", diff.index), "current scene does not match the planned deletion target"));
    }
    if diff.default_scene_after != remap_default(base.document.scene, diff.index) {
        return Err(reject("gltf.mutation.invalid-reference-repair", "diff/defaultSceneAfter", "default scene remap must match the deletion index"));
    }
    Ok(())
}
pub fn apply(diff: &GltfDeleteSceneDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(diff, base)?;
    let mut next = base.clone();
    scenes_op(&mut next, GltfTopLevelFamily::Scenes, diff.index, None, None)?;
    if next.document.scene != diff.default_scene_after {
        return Err(reject("gltf.mutation.reference-repair-failed", "document/scene", "default scene did not remap as planned"));
    }
    Ok(next)
}
pub fn encode(diff: &GltfDeleteSceneDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> {
    serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string()))
}
pub fn derive(base: &GltfSnapshot, index: usize) -> Result<GltfDeleteSceneDiff, GltfTopLevelMutationRejection> {
    if index >= base.document.scenes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "index must address a scene"));
    }
    if base.document.scene.is_some_and(|scene| scene >= base.document.scenes.len()) {
        return Err(reject("gltf.reference.invalid-default-scene", "document/scene", "default scene must address an existing scene"));
    }
    let after = remap_default(base.document.scene, index);
    Ok(GltfDeleteSceneDiff { id: ID.into(), version: 1, phase: GltfDeleteSceneDiffPhase::Diff, touched_paths: paths(base.document.scene, after, index), index, deleted_scene: base.document.scenes[index].clone(), default_scene_after: after })
}
