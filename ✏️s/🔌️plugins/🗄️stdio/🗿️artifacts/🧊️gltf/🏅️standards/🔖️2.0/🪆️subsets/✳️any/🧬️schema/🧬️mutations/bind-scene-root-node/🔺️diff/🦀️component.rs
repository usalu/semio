//! 🔺️ Minimal deterministic scene-root insertion patch.
//!
use crate::artifacts::gltf::schema::mutations::bind_scene_root_node::mutation::{validate, GltfBindSceneRootNodePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindSceneRootNodeDiff {
    pub scene: usize,
    pub node: usize,
    pub position: usize,
    pub touched_paths: Vec<String>,
}
pub async fn derive(payload: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfBindSceneRootNodeDiff, GltfTopLevelMutationRejection> {
    validate(payload, base).await?;
    Ok(GltfBindSceneRootNodeDiff { scene: payload.scene, node: payload.node, position: payload.position, touched_paths: vec![format!("document/scenes/{}/nodes/{}", payload.scene, payload.position)] })
}
pub async fn apply(base: &GltfSnapshot, diff: &GltfBindSceneRootNodeDiff) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    let path = format!("document/scenes/{}/nodes/{}", diff.scene, diff.position);
    if diff.touched_paths.len() != 1 || diff.touched_paths[0] != path {
        return Err(reject("gltf.mutation.invalid-touched-path", path, "patch touched path does not match its root coordinates").await);
    }
    let scene = base.document.scenes.get(diff.scene).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/scenes", "scene is absent"))?;
    if diff.position > scene.nodes.len() || scene.nodes.contains(&diff.node) || base.document.nodes.get(diff.node).is_none() {
        return Err(reject("gltf.mutation.stale-diff", format!("document/scenes/{}/nodes/{}", diff.scene, diff.position), "scene, position, or node identity is stale").await);
    }
    let mut next = base.clone();
    next.document.scenes[diff.scene].nodes.insert(diff.position, diff.node);
    Ok(next)
}
pub async fn encode(diff: &GltfBindSceneRootNodeDiff) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(diff)
}
