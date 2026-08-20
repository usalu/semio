//! 🔺️ bind-node-mesh direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::bind_node_mesh::mutation::{validate,GltfBindNodeMeshPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfBindNodeMeshDiff{pub operation:GltfBindNodeMeshPayload,pub after:Option<usize>,pub touched_paths:Vec<String>}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(operation:&GltfBindNodeMeshPayload,base:&GltfSnapshot)->Result<GltfBindNodeMeshDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=Some(operation.mesh);Ok(GltfBindNodeMeshDiff{operation:operation.clone(),after,touched_paths:["document/nodes/*/mesh"].into_iter().map(str::to_owned).collect()})}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(base:&GltfSnapshot,diff:&GltfBindNodeMeshDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].mesh=diff.after;Ok(next)}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff:&GltfBindNodeMeshDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
