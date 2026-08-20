//! 🔺️ change-scene-extension-data direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_scene_extension_data::mutation::{validate,GltfChangeSceneExtensionDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeSceneExtensionDataDiff{pub operation:GltfChangeSceneExtensionDataPayload,pub after:Option<GltfJson>,pub touched_paths:Vec<String>}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(operation:&GltfChangeSceneExtensionDataPayload,base:&GltfSnapshot)->Result<GltfChangeSceneExtensionDataDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=match &operation.data{crate::artifacts::gltf::schema::mutations::change_scene_extension_data::mutation::GltfDataPresence::Absent=>None,crate::artifacts::gltf::schema::mutations::change_scene_extension_data::mutation::GltfDataPresence::Present{value}=>Some(value.clone())};Ok(GltfChangeSceneExtensionDataDiff{operation:operation.clone(),after,touched_paths:["document/scenes/*/extensions"].into_iter().map(str::to_owned).collect()})}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(base:&GltfSnapshot,diff:&GltfChangeSceneExtensionDataDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.scenes[diff.operation.scene].extensions=diff.after.clone();Ok(next)}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff:&GltfChangeSceneExtensionDataDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
