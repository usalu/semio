//! 🦀 bind-scene-root-node: typed validation and atomic application.
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.bind-scene-root-node.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindSceneRootNodePayload {
    pub scene: usize,
    pub node: usize,
    pub position: usize,
}

pub async fn validate(payload: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    checked_position(payload.position, base.document.scenes[payload.scene].nodes.len(), "document/scenes/nodes")?;
    if base.document.scenes[payload.scene].nodes.contains(&payload.node) {
        return Err(reject("gltf.mutation.duplicate-scene-root", format!("document/scenes/{}/nodes", payload.scene), "node is already a scene root"));
    }
    Ok(())
}

pub async fn apply(payload: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let mut next = base.clone();
    next.document.scenes[payload.scene].nodes.insert(payload.position, payload.node);
    Ok(next)
}
