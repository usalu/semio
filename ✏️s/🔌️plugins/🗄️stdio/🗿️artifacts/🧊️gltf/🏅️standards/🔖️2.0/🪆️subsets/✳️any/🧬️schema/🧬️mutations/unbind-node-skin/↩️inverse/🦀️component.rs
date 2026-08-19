//! ↩️ unbind-node-skin exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::unbind_node_skin::mutation::{validate,GltfUnbindNodeSkinPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfUnbindNodeSkinInverse{pub operation:GltfUnbindNodeSkinPayload,pub before:Option<usize>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfUnbindNodeSkinPayload,base:&GltfSnapshot)->Result<GltfUnbindNodeSkinInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.nodes[operation.node].skin;Ok(GltfUnbindNodeSkinInverse{operation:operation.clone(),before,touched_paths:["document/nodes/*/skin"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,inverse:&GltfUnbindNodeSkinInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].skin=inverse.before;Ok(next)}
pub async fn encode(inverse:&GltfUnbindNodeSkinInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
