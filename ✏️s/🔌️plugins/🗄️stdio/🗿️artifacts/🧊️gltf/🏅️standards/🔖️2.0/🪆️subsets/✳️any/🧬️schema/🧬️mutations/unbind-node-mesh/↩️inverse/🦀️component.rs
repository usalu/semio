//! ↩️ unbind-node-mesh exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::unbind_node_mesh::mutation::{validate,GltfUnbindNodeMeshPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfUnbindNodeMeshInverse{pub operation:GltfUnbindNodeMeshPayload,pub before:Option<usize>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfUnbindNodeMeshPayload,base:&GltfSnapshot)->Result<GltfUnbindNodeMeshInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.nodes[operation.node].mesh;Ok(GltfUnbindNodeMeshInverse{operation:operation.clone(),before,touched_paths:["document/nodes/*/mesh"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,inverse:&GltfUnbindNodeMeshInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].mesh=inverse.before;Ok(next)}
pub async fn encode(inverse:&GltfUnbindNodeMeshInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
