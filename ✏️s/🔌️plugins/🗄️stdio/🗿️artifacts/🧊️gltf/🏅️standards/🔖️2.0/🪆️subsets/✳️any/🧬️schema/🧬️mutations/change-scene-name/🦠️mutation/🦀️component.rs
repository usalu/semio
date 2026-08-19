//! 🦀 change-scene-name: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.change-scene-name.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeSceneNamePayload { pub scene: usize, pub value: Option<String> }
pub async fn validate(payload: &GltfChangeSceneNamePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?; Ok(()) }
pub async fn apply(payload: &GltfChangeSceneNamePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.scenes[payload.scene].name = payload.value.clone(); Ok(next) }
