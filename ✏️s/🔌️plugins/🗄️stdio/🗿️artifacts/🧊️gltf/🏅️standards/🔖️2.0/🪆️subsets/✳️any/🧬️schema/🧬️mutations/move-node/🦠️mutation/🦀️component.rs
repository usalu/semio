//! 🦠️ move-node typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-node.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/nodes"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveNodePayload { pub index: usize, pub position: usize }
pub fn validate(payload: &GltfMoveNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.nodes.len() || payload.position >= base.document.nodes.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/nodes", "indices must address items")); } if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/nodes", "destination equals source")); }  Ok(()) }
pub fn apply(payload: &GltfMoveNodePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); nodes_op(&mut next, GltfTopLevelFamily::Nodes, payload.index, Some(payload.position), None)?;  Ok(next) }
