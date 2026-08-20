//! ↩️ unbind-node-camera exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::unbind_node_camera::mutation::{validate,GltfUnbindNodeCameraPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfUnbindNodeCameraInverse{pub operation:GltfUnbindNodeCameraPayload,pub before:Option<usize>,pub touched_paths:Vec<String>}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(operation:&GltfUnbindNodeCameraPayload,base:&GltfSnapshot)->Result<GltfUnbindNodeCameraInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.nodes[operation.node].camera;Ok(GltfUnbindNodeCameraInverse{operation:operation.clone(),before,touched_paths:["document/nodes/*/camera"].into_iter().map(str::to_owned).collect()})}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(base:&GltfSnapshot,inverse:&GltfUnbindNodeCameraInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].camera=inverse.before;Ok(next)}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(inverse:&GltfUnbindNodeCameraInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
