//! 🦠️ reorder-nodes typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-nodes.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/nodes"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderNodesPayload { pub order: Vec<usize> }
pub fn validate(payload: &GltfReorderNodesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.nodes.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.nodes.len()) { return Err(reject("gltf.mutation.invalid-permutation", "document/nodes", "order must contain every index once")); } if payload.order.iter().enumerate().all(|(index, value)| index == *value) { return Err(reject("gltf.mutation.no-observable-change", "document/nodes", "order already matches")); }  Ok(()) }
pub fn apply(payload: &GltfReorderNodesPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); nodes_op(&mut next, GltfTopLevelFamily::Nodes, payload.order[0], None, Some(&payload.order))?;  Ok(next) }
