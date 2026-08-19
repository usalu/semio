//! 🔺️ change-scene-extra-data direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_scene_extra_data::mutation::{validate,GltfChangeSceneExtraDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeSceneExtraDataDiff{pub operation:GltfChangeSceneExtraDataPayload,pub after:Option<GltfJson>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfChangeSceneExtraDataPayload,base:&GltfSnapshot)->Result<GltfChangeSceneExtraDataDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=match &operation.data{crate::artifacts::gltf::schema::mutations::change_scene_extra_data::mutation::GltfDataPresence::Absent=>None,crate::artifacts::gltf::schema::mutations::change_scene_extra_data::mutation::GltfDataPresence::Present{value}=>Some(value.clone())};Ok(GltfChangeSceneExtraDataDiff{operation:operation.clone(),after,touched_paths:["document/scenes/*/extras"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,diff:&GltfChangeSceneExtraDataDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.scenes[diff.operation.scene].extras=diff.after.clone();Ok(next)}
pub async fn encode(diff:&GltfChangeSceneExtraDataDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
