//! ↩️ Exact scene-root removal inverse.
//!
use crate::artifacts::gltf::schema::mutations::bind_scene_root_node::mutation::{validate, GltfBindSceneRootNodePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindSceneRootNodeInverse {
    pub scene: usize,
    pub node: usize,
    pub position: usize,
    pub expected_nodes: Vec<usize>,
    pub touched_paths: Vec<String>,
}
pub async fn derive(payload: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfBindSceneRootNodeInverse, GltfTopLevelMutationRejection> {
    validate(payload, base).await?;
    let after = crate::artifacts::gltf::schema::mutations::bind_scene_root_node::mutation::apply(payload, base).await?;
    Ok(GltfBindSceneRootNodeInverse {
        scene: payload.scene,
        node: payload.node,
        position: payload.position,
        expected_nodes: after.document.scenes[payload.scene].nodes.clone(),
        touched_paths: vec![format!("document/scenes/{}/nodes/{}", payload.scene, payload.position)],
    })
}
pub async fn apply(base: &GltfSnapshot, inverse: &GltfBindSceneRootNodeInverse) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    let path = format!("document/scenes/{}/nodes/{}", inverse.scene, inverse.position);
    if inverse.touched_paths.len() != 1 || inverse.touched_paths[0] != path {
        return Err(reject("gltf.mutation.invalid-touched-path", path, "inverse touched path does not match its root coordinates").await);
    }
    let nodes = &base.document.scenes.get(inverse.scene).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/scenes", "scene is absent"))?.nodes;
    if *nodes != inverse.expected_nodes || nodes.get(inverse.position) != Some(&inverse.node) {
        return Err(reject("gltf.mutation.stale-inverse", format!("document/scenes/{}/nodes/{}", inverse.scene, inverse.position), "scene roots do not equal the planned forward state").await);
    }
    let mut next = base.clone();
    next.document.scenes[inverse.scene].nodes.remove(inverse.position);
    Ok(next)
}
pub async fn encode(inverse: &GltfBindSceneRootNodeInverse) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(inverse)
}
