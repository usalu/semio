//! ↩️ change-scene-extension-data exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_scene_extension_data::mutation::{validate,GltfChangeSceneExtensionDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeSceneExtensionDataInverse{pub operation:GltfChangeSceneExtensionDataPayload,pub before:Option<GltfJson>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfChangeSceneExtensionDataPayload,base:&GltfSnapshot)->Result<GltfChangeSceneExtensionDataInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.scenes[operation.scene].extensions.clone();Ok(GltfChangeSceneExtensionDataInverse{operation:operation.clone(),before,touched_paths:["document/scenes/*/extensions"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,inverse:&GltfChangeSceneExtensionDataInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.scenes[diff.operation.scene].extensions=inverse.before.clone();Ok(next)}
pub async fn encode(inverse:&GltfChangeSceneExtensionDataInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
