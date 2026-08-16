//! ↩️ Exact create-scene removal inverse with complete default-scene restoration.
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, scenes_op, GltfTopLevelFamily, GltfTopLevelMutationRejection};
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
#[serde(rename_all = "camelCase")]
pub struct GltfCreateSceneInverse {
    pub id: String,
    pub version: u32,
    pub phase: GltfCreateSceneInversePhase,
    pub touched_paths: Vec<String>,
    pub position: usize,
    pub expected_scene: GltfScene,
    pub default_scene_before: Option<usize>,
    pub expected_default_scene_after: Option<usize>,
}
fn default_after(default_scene: Option<usize>, position: usize) -> Result<Option<usize>, GltfTopLevelMutationRejection> {
    default_scene.map(|scene| if scene >= position { scene.checked_add(1).ok_or_else(|| reject("gltf.mutation.reference-overflow", "document/scene", "default scene cannot be remapped beyond usize")) } else { Ok(scene) }).transpose()
}
fn paths(position: usize, before: Option<usize>, after: Option<usize>) -> Vec<String> {
    if before == after {
        vec![format!("document/scenes/{}", position)]
    } else {
        vec![format!("document/scenes/{}", position), "document/scene".into()]
    }
}
pub fn validate(inverse: &GltfCreateSceneInverse, after: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if inverse.id != ID || inverse.version != 1 || inverse.phase != GltfCreateSceneInversePhase::Inverse {
        return Err(reject("gltf.mutation.invalid-inverse-envelope", "inverse", "canonical identity or phase does not match"));
    }
    if inverse.position >= after.document.scenes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "position must address the created scene"));
    }
    if inverse.touched_paths != paths(inverse.position, inverse.default_scene_before, inverse.expected_default_scene_after) {
        return Err(reject("gltf.mutation.invalid-touched-paths", "inverse/touchedPaths", "paths must name every concrete changed location"));
    }
    if inverse.expected_scene != GltfScene::default() {
        return Err(reject("gltf.mutation.invalid-created-scene", "inverse/expectedScene", "inverse must target the canonical empty scene"));
    }
    if inverse.expected_default_scene_after != after.document.scene {
        return Err(reject("gltf.mutation.stale-inverse", "document/scene", "default scene does not match the forward-created state"));
    }
    if after.document.scenes[inverse.position] != inverse.expected_scene {
        return Err(reject("gltf.mutation.stale-inverse", format!("document/scenes/{}", inverse.position), "current scene does not match the forward-created scene"));
    }
    Ok(())
}
pub fn apply(inverse: &GltfCreateSceneInverse, after: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(inverse, after)?;
    let mut next = after.clone();
    scenes_op(&mut next, GltfTopLevelFamily::Scenes, inverse.position, None, None)?;
    next.document.scene = inverse.default_scene_before;
    Ok(next)
}
pub fn encode(inverse: &GltfCreateSceneInverse) -> Result<Vec<u8>, GltfTopLevelMutationRejection> {
    serde_json::to_vec(inverse).map_err(|error| reject("gltf.mutation.encode-failed", "inverse", error.to_string()))
}
pub fn derive(base: &GltfSnapshot, position: usize) -> Result<GltfCreateSceneInverse, GltfTopLevelMutationRejection> {
    if position > base.document.scenes.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection"));
    }
    let default_scene_before = base.document.scene;
    let expected_default_scene_after = default_after(default_scene_before, position)?;
    Ok(GltfCreateSceneInverse {
        id: ID.into(),
        version: 1,
        phase: GltfCreateSceneInversePhase::Inverse,
        touched_paths: paths(position, default_scene_before, expected_default_scene_after),
        position,
        expected_scene: GltfScene::default(),
        default_scene_before,
        expected_default_scene_after,
    })
}
