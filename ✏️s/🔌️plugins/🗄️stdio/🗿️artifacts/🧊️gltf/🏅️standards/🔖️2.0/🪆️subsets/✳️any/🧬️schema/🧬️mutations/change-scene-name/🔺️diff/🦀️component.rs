//! 🔺️ change-scene-name direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_scene_name::mutation::{validate,GltfChangeSceneNamePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeSceneNameDiff{pub operation:GltfChangeSceneNamePayload,pub after:Option<String>,pub touched_paths:Vec<String>}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(operation:&GltfChangeSceneNamePayload,base:&GltfSnapshot)->Result<GltfChangeSceneNameDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=operation.value.clone();Ok(GltfChangeSceneNameDiff{operation:operation.clone(),after,touched_paths:["document/scenes/*/name"].into_iter().map(str::to_owned).collect()})}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(base:&GltfSnapshot,diff:&GltfChangeSceneNameDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.scenes[diff.operation.scene].name=diff.after.clone();Ok(next)}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff:&GltfChangeSceneNameDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
