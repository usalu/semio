//! ↩️ bind-node-skin exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::bind_node_skin::mutation::{validate,GltfBindNodeSkinPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfBindNodeSkinInverse{pub operation:GltfBindNodeSkinPayload,pub before:Option<usize>,pub touched_paths:Vec<String>}
pub fn derive(operation:&GltfBindNodeSkinPayload,base:&GltfSnapshot)->Result<GltfBindNodeSkinInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.nodes[operation.node].skin;Ok(GltfBindNodeSkinInverse{operation:operation.clone(),before,touched_paths:["document/nodes/*/skin"].into_iter().map(str::to_owned).collect()})}
pub fn apply(base:&GltfSnapshot,inverse:&GltfBindNodeSkinInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].skin=inverse.before;Ok(next)}
pub fn encode(inverse:&GltfBindNodeSkinInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
