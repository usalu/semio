//! 🦀 change-node-extension-data: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.change-node-extension-data.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum GltfDataPresence { Absent, Present { value: GltfJson } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeNodeExtensionDataPayload { pub node: usize, pub data: GltfDataPresence }
pub async fn validate(payload: &GltfChangeNodeExtensionDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.node, base.document.nodes.len(), "document/nodes")?; Ok(()) }
pub async fn apply(payload: &GltfChangeNodeExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.nodes[payload.node].extensions = match &payload.data { GltfDataPresence::Absent => None, GltfDataPresence::Present { value } => Some(value.clone()) }; Ok(next) }
