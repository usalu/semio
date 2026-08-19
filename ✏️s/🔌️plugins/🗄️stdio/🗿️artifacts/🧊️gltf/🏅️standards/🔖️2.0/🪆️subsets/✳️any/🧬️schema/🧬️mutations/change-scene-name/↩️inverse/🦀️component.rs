//! ↩️ change-scene-name exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_scene_name::mutation::{validate,GltfChangeSceneNamePayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeSceneNameInverse{pub operation:GltfChangeSceneNamePayload,pub before:Option<String>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfChangeSceneNamePayload,base:&GltfSnapshot)->Result<GltfChangeSceneNameInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.scenes[operation.scene].name.clone();Ok(GltfChangeSceneNameInverse{operation:operation.clone(),before,touched_paths:["document/scenes/*/name"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,inverse:&GltfChangeSceneNameInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.scenes[diff.operation.scene].name=inverse.before.clone();Ok(next)}
pub async fn encode(inverse:&GltfChangeSceneNameInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
