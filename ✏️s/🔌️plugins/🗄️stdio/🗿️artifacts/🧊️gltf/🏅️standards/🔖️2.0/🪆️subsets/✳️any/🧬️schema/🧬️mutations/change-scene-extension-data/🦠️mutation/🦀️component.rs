//! 🦀 change-scene-extension-data: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.change-scene-extension-data.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum GltfDataPresence { Absent, Present { value: GltfJson } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeSceneExtensionDataPayload { pub scene: usize, pub data: GltfDataPresence }
pub async fn validate(payload: &GltfChangeSceneExtensionDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?; Ok(()) }
pub async fn apply(payload: &GltfChangeSceneExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.scenes[payload.scene].extensions = match &payload.data { GltfDataPresence::Absent => None, GltfDataPresence::Present { value } => Some(value.clone()) }; Ok(next) }
