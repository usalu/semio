//! 🦀 unbind-node-mesh: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.unbind-node-mesh.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfUnbindNodeMeshPayload { pub node: usize }
pub fn validate(payload: &GltfUnbindNodeMeshPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.node, base.document.nodes.len(), "document/nodes")?; if base.document.nodes[payload.node].mesh.is_none() { return Err(reject("gltf.mutation.relation-absent", format!("document/nodes/{}/mesh", payload.node), "node has no binding")); } Ok(()) }
pub fn apply(payload: &GltfUnbindNodeMeshPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.nodes[payload.node].mesh = None; Ok(next) }
