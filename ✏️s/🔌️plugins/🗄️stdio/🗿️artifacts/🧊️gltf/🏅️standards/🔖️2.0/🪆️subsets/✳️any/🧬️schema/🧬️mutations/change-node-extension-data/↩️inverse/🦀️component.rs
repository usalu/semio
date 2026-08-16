//! ↩️ change-node-extension-data exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_node_extension_data::mutation::{validate,GltfChangeNodeExtensionDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeNodeExtensionDataInverse{pub operation:GltfChangeNodeExtensionDataPayload,pub before:Option<GltfJson>,pub touched_paths:Vec<String>}
pub fn derive(operation:&GltfChangeNodeExtensionDataPayload,base:&GltfSnapshot)->Result<GltfChangeNodeExtensionDataInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.nodes[operation.node].extensions.clone();Ok(GltfChangeNodeExtensionDataInverse{operation:operation.clone(),before,touched_paths:["document/nodes/*/extensions"].into_iter().map(str::to_owned).collect()})}
pub fn apply(base:&GltfSnapshot,inverse:&GltfChangeNodeExtensionDataInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].extensions=inverse.before.clone();Ok(next)}
pub fn encode(inverse:&GltfChangeNodeExtensionDataInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
