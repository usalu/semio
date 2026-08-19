//! 🦀 move-scene-root-node: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.move-scene-root-node.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMoveSceneRootNodePayload { pub scene: usize, pub node: usize, pub position: usize }
pub async fn validate(payload: &GltfMoveSceneRootNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?; let roots = &base.document.scenes[payload.scene].nodes; let index = roots.iter().position(|node| *node == payload.node).ok_or_else(|| reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", payload.scene), "node is not a root of this scene"))?; checked_index(payload.position, roots.len(), "document/scenes/nodes")?; if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/scenes/nodes", "destination equals source")); } Ok(()) }
pub async fn apply(payload: &GltfMoveSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let roots = &mut next.document.scenes[payload.scene].nodes; let index = roots.iter().position(|node| *node == payload.node).expect("validated root"); let node = roots.remove(index); roots.insert(payload.position, node); Ok(next) }
