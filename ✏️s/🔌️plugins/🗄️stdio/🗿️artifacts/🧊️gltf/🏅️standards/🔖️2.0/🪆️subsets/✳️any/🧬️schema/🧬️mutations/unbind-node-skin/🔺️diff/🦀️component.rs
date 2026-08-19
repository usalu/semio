//! 🔺️ unbind-node-skin direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::unbind_node_skin::mutation::{validate,GltfUnbindNodeSkinPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfUnbindNodeSkinDiff{pub operation:GltfUnbindNodeSkinPayload,pub after:Option<usize>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfUnbindNodeSkinPayload,base:&GltfSnapshot)->Result<GltfUnbindNodeSkinDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=None;Ok(GltfUnbindNodeSkinDiff{operation:operation.clone(),after,touched_paths:["document/nodes/*/skin"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,diff:&GltfUnbindNodeSkinDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].skin=diff.after;Ok(next)}
pub async fn encode(diff:&GltfUnbindNodeSkinDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
