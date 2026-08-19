//! ↩️ bind-node-camera exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::bind_node_camera::mutation::{validate,GltfBindNodeCameraPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfBindNodeCameraInverse{pub operation:GltfBindNodeCameraPayload,pub before:Option<usize>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfBindNodeCameraPayload,base:&GltfSnapshot)->Result<GltfBindNodeCameraInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.nodes[operation.node].camera;Ok(GltfBindNodeCameraInverse{operation:operation.clone(),before,touched_paths:["document/nodes/*/camera"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,inverse:&GltfBindNodeCameraInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].camera=inverse.before;Ok(next)}
pub async fn encode(inverse:&GltfBindNodeCameraInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
