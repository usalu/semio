//! 🦀 change-node-extra-data: typed validation and atomic application.
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::checked_index;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::snapshot::GltfJson;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.change-node-extra-data.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum GltfDataPresence { Absent, Present { value: GltfJson } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeNodeExtraDataPayload { pub node: usize, pub data: GltfDataPresence }
pub fn validate(payload: &GltfChangeNodeExtraDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.node, base.document.nodes.len(), "document/nodes")?; let unchanged = match &payload.data { GltfDataPresence::Absent => base.document.nodes[payload.node].extras.is_none(), GltfDataPresence::Present { value } => base.document.nodes[payload.node].extras.as_ref() == Some(value) }; if unchanged { return Err(reject("gltf.mutation.no-observable-change", format!("document/nodes/{}/extras", payload.node), "extras already has the requested presence and value")); } Ok(()) }
pub fn apply(payload: &GltfChangeNodeExtraDataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.nodes[payload.node].extras = match &payload.data { GltfDataPresence::Absent => None, GltfDataPresence::Present { value } => Some(value.clone()) }; Ok(next) }
