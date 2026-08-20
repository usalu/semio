//! 🔺️ bind-node-skin direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::bind_node_skin::mutation::{validate,GltfBindNodeSkinPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfBindNodeSkinDiff{pub operation:GltfBindNodeSkinPayload,pub after:Option<usize>,pub touched_paths:Vec<String>}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(operation:&GltfBindNodeSkinPayload,base:&GltfSnapshot)->Result<GltfBindNodeSkinDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=Some(operation.skin);Ok(GltfBindNodeSkinDiff{operation:operation.clone(),after,touched_paths:["document/nodes/*/skin"].into_iter().map(str::to_owned).collect()})}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(base:&GltfSnapshot,diff:&GltfBindNodeSkinDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].skin=diff.after;Ok(next)}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff:&GltfBindNodeSkinDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
