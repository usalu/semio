//! 🔺️ Exact scene-root removal patch.
//!
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::mutations::unbind_scene_root_node::mutation::{validate, GltfUnbindSceneRootNodePayload};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfUnbindSceneRootNodeDiff {
    pub scene: usize,
    pub node: usize,
    pub position: usize,
    pub touched_paths: Vec<String>,
}

pub async fn derive(payload: &GltfUnbindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfUnbindSceneRootNodeDiff, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let position =
        base.document.scenes[payload.scene].nodes.iter().position(|node| *node == payload.node).ok_or_else(|| reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", payload.scene), "node is not a root of this scene"))?;
    Ok(GltfUnbindSceneRootNodeDiff { scene: payload.scene, node: payload.node, position, touched_paths: vec![format!("document/scenes/{}/nodes/{}", payload.scene, position)] })
}

pub async fn apply(base: &GltfSnapshot, diff: &GltfUnbindSceneRootNodeDiff) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    let path = format!("document/scenes/{}/nodes/{}", diff.scene, diff.position);
    if diff.touched_paths.len() != 1 || diff.touched_paths[0] != path {
        return Err(reject("gltf.mutation.invalid-touched-path", path, "patch touched path does not match its root coordinates"));
    }
    let nodes = &base.document.scenes.get(diff.scene).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/scenes", "scene is absent"))?.nodes;
    if nodes.get(diff.position) != Some(&diff.node) {
        return Err(reject("gltf.mutation.stale-diff", format!("document/scenes/{}/nodes/{}", diff.scene, diff.position), "node is not at the recorded removal position"));
    }
    let mut next = base.clone();
    next.document.scenes[diff.scene].nodes.remove(diff.position);
    Ok(next)
}

pub async fn encode(diff: &GltfUnbindSceneRootNodeDiff) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(diff)
}
