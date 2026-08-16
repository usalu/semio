//! 🔺️ unbind-node-camera direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::unbind_node_camera::mutation::{validate,GltfUnbindNodeCameraPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfUnbindNodeCameraDiff{pub operation:GltfUnbindNodeCameraPayload,pub after:Option<usize>,pub touched_paths:Vec<String>}
pub fn derive(operation:&GltfUnbindNodeCameraPayload,base:&GltfSnapshot)->Result<GltfUnbindNodeCameraDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=None;Ok(GltfUnbindNodeCameraDiff{operation:operation.clone(),after,touched_paths:["document/nodes/*/camera"].into_iter().map(str::to_owned).collect()})}
pub fn apply(base:&GltfSnapshot,diff:&GltfUnbindNodeCameraDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].camera=diff.after;Ok(next)}
pub fn encode(diff:&GltfUnbindNodeCameraDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
