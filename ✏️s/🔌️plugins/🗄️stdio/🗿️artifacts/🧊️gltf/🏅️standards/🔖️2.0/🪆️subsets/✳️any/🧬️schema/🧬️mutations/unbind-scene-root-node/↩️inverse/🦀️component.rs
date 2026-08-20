//! ↩️ Exact scene-root reinsertion inverse.
//!
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::mutations::unbind_scene_root_node::mutation::{validate, GltfUnbindSceneRootNodePayload};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfUnbindSceneRootNodeInverse {
    pub scene: usize,
    pub node: usize,
    pub position: usize,
    pub expected_nodes: Vec<usize>,
    pub touched_paths: Vec<String>,
}

pub async fn derive(payload: &GltfUnbindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfUnbindSceneRootNodeInverse, GltfTopLevelMutationRejection> {
    validate(payload, base).await?;
    let position =
        base.document.scenes[payload.scene].nodes.iter().position(|node| *node == payload.node).ok_or_else(|| reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", payload.scene), "node is not a root of this scene"))?;
    let after = crate::artifacts::gltf::schema::mutations::unbind_scene_root_node::mutation::apply(payload, base).await?;
    Ok(GltfUnbindSceneRootNodeInverse { scene: payload.scene, node: payload.node, position, expected_nodes: after.document.scenes[payload.scene].nodes.clone(), touched_paths: vec![format!("document/scenes/{}/nodes/{}", payload.scene, position)] })
}

pub async fn apply(base: &GltfSnapshot, inverse: &GltfUnbindSceneRootNodeInverse) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    let path = format!("document/scenes/{}/nodes/{}", inverse.scene, inverse.position);
    if inverse.touched_paths.len() != 1 || inverse.touched_paths[0] != path {
        return Err(reject("gltf.mutation.invalid-touched-path", path, "inverse touched path does not match its root coordinates").await);
    }
    let nodes = &base.document.scenes.get(inverse.scene).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/scenes", "scene is absent"))?.nodes;
    if *nodes != inverse.expected_nodes || inverse.position > nodes.len() || nodes.contains(&inverse.node) {
        return Err(reject("gltf.mutation.stale-inverse", format!("document/scenes/{}/nodes/{}", inverse.scene, inverse.position), "scene roots do not equal the planned forward state").await);
    }
    let mut next = base.clone();
    next.document.scenes[inverse.scene].nodes.insert(inverse.position, inverse.node);
    Ok(next)
}

pub async fn encode(inverse: &GltfUnbindSceneRootNodeInverse) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(inverse)
}
