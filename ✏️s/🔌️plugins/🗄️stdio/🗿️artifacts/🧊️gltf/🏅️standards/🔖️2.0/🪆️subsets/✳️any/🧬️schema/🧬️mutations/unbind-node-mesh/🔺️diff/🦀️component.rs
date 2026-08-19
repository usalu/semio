//! 🔺️ unbind-node-mesh direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::unbind_node_mesh::mutation::{validate,GltfUnbindNodeMeshPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfUnbindNodeMeshDiff{pub operation:GltfUnbindNodeMeshPayload,pub after:Option<usize>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfUnbindNodeMeshPayload,base:&GltfSnapshot)->Result<GltfUnbindNodeMeshDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=None;Ok(GltfUnbindNodeMeshDiff{operation:operation.clone(),after,touched_paths:["document/nodes/*/mesh"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,diff:&GltfUnbindNodeMeshDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].mesh=diff.after;Ok(next)}
pub async fn encode(diff:&GltfUnbindNodeMeshDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
