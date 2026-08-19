//! ↩️ Exact delete-scene inverse restores the deleted scene and default-scene repair state.
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.delete-scene.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GltfDeleteSceneInversePhase {
    Inverse,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfDeleteSceneInverse {
    pub id: String,
    pub version: u32,
    pub phase: GltfDeleteSceneInversePhase,
    pub touched_paths: Vec<String>,
    pub index: usize,
    pub deleted_scene: GltfScene,
    pub default_scene_before: Option<usize>,
    pub expected_default_scene_after: Option<usize>,
}
async fn paths(before: Option<usize>, after: Option<usize>, index: usize) -> Vec<String> {
    let mut paths = vec![format!("document/scenes/{}", index)];
    if before != after {
        paths.push("document/scene".into());
    }
    paths
}
pub async fn validate(inverse: &GltfDeleteSceneInverse, after: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if inverse.id != ID || inverse.version != 1 || inverse.phase != GltfDeleteSceneInversePhase::Inverse {
        return Err(reject("gltf.mutation.invalid-inverse-envelope", "inverse", "canonical identity or phase does not match"));
    }
    if inverse.index > after.document.scenes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "index must address an insertion location"));
    }
    if inverse.default_scene_before.is_some_and(|scene| scene >= after.document.scenes.len() + 1) {
        return Err(reject("gltf.reference.invalid-default-scene", "inverse/defaultSceneBefore", "restored default scene must address the restored collection"));
    }
    if inverse.expected_default_scene_after.is_some_and(|scene| scene >= after.document.scenes.len()) {
        return Err(reject("gltf.reference.invalid-default-scene", "inverse/expectedDefaultSceneAfter", "expected repaired default scene must address the current collection"));
    }
    if inverse.touched_paths != paths(inverse.default_scene_before, inverse.expected_default_scene_after, inverse.index) {
        return Err(reject("gltf.mutation.invalid-touched-paths", "inverse/touchedPaths", "paths must be concrete deletion effects"));
    }
    if after.document.scene != inverse.expected_default_scene_after {
        return Err(reject("gltf.mutation.stale-inverse", "document/scene", "current default scene does not match the deletion repair state"));
    }
    Ok(())
}
pub async fn apply(inverse: &GltfDeleteSceneInverse, after: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(inverse, after)?;
    let mut next = after.clone();
    repair(&mut next.document, GltfTopLevelFamily::Scenes, &Change::Insert(inverse.index))?;
    next.document.scenes.insert(inverse.index, inverse.deleted_scene.clone());
    next.document.scene = inverse.default_scene_before;
    Ok(next)
}
pub async fn encode(inverse: &GltfDeleteSceneInverse) -> Result<Vec<u8>, GltfTopLevelMutationRejection> {
    serde_json::to_vec(inverse).map_err(|error| reject("gltf.mutation.encode-failed", "inverse", error.to_string()))
}
pub async fn derive(base: &GltfSnapshot, index: usize) -> Result<GltfDeleteSceneInverse, GltfTopLevelMutationRejection> {
    if index >= base.document.scenes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "index must address a scene"));
    }
    if base.document.scene.is_some_and(|scene| scene >= base.document.scenes.len()) {
        return Err(reject("gltf.reference.invalid-default-scene", "document/scene", "default scene must address an existing scene"));
    }
    let before = base.document.scene;
    let after = match before {
        None => None,
        Some(scene) if scene == index => None,
        Some(scene) if scene > index => Some(scene - 1),
        Some(scene) => Some(scene),
    };
    Ok(GltfDeleteSceneInverse {
        id: ID.into(),
        version: 1,
        phase: GltfDeleteSceneInversePhase::Inverse,
        touched_paths: paths(before, after, index),
        index,
        deleted_scene: base.document.scenes[index].clone(),
        default_scene_before: before,
        expected_default_scene_after: after,
    })
}
