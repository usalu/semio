//! 🔺️ change-node-extension-data direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_node_extension_data::mutation::{validate,GltfChangeNodeExtensionDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeNodeExtensionDataDiff{pub operation:GltfChangeNodeExtensionDataPayload,pub after:Option<GltfJson>,pub touched_paths:Vec<String>}
pub fn derive(operation:&GltfChangeNodeExtensionDataPayload,base:&GltfSnapshot)->Result<GltfChangeNodeExtensionDataDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=match &operation.data{crate::artifacts::gltf::schema::mutations::change_node_extension_data::mutation::GltfDataPresence::Absent=>None,crate::artifacts::gltf::schema::mutations::change_node_extension_data::mutation::GltfDataPresence::Present{value}=>Some(value.clone())};Ok(GltfChangeNodeExtensionDataDiff{operation:operation.clone(),after,touched_paths:["document/nodes/*/extensions"].into_iter().map(str::to_owned).collect()})}
pub fn apply(base:&GltfSnapshot,diff:&GltfChangeNodeExtensionDataDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].extensions=diff.after.clone();Ok(next)}
pub fn encode(diff:&GltfChangeNodeExtensionDataDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
