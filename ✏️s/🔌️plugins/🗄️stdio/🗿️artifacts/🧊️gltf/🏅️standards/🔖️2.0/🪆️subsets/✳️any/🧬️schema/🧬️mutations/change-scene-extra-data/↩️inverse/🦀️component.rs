//! ↩️ change-scene-extra-data exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_scene_extra_data::mutation::{validate,GltfChangeSceneExtraDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeSceneExtraDataInverse{pub operation:GltfChangeSceneExtraDataPayload,pub before:Option<GltfJson>,pub touched_paths:Vec<String>}
pub fn derive(operation:&GltfChangeSceneExtraDataPayload,base:&GltfSnapshot)->Result<GltfChangeSceneExtraDataInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.scenes[operation.scene].extras.clone();Ok(GltfChangeSceneExtraDataInverse{operation:operation.clone(),before,touched_paths:["document/scenes/*/extras"].into_iter().map(str::to_owned).collect()})}
pub fn apply(base:&GltfSnapshot,inverse:&GltfChangeSceneExtraDataInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.scenes[diff.operation.scene].extras=inverse.before.clone();Ok(next)}
pub fn encode(inverse:&GltfChangeSceneExtraDataInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
