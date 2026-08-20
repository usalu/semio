//! 🦀 change-node-name: typed validation and atomic application.
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::checked_index;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.change-node-name.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeNodeNamePayload { pub node: usize, pub value: Option<String> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.node, base.document.nodes.len(), "document/nodes")?; if base.document.nodes[payload.node].name == payload.value { return Err(reject("gltf.mutation.no-observable-change", format!("document/nodes/{}/name", payload.node), "name already has the requested presence and value")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.nodes[payload.node].name = payload.value.clone(); Ok(next) }
