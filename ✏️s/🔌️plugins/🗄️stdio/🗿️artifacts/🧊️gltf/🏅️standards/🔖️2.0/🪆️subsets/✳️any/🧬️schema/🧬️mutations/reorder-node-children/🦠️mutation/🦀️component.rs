//! 🦀 reorder-node-children: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-node-children.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfReorderNodeChildrenPayload { pub parent: usize, pub order: Vec<usize> }
pub async fn validate(payload: &GltfReorderNodeChildrenPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?; let children = &base.document.nodes[payload.parent].children; if payload.order.len() != children.len() || payload.order.iter().any(|child| !children.contains(child)) || { let mut order = payload.order.clone(); order.sort_unstable(); order.dedup(); order.len() != children.len() } { return Err(reject("gltf.mutation.invalid-permutation", "document/nodes/children", "order must contain every child identity once")); } if payload.order == *children { return Err(reject("gltf.mutation.no-observable-change", "document/nodes/children", "reorder must change order")); } Ok(()) }
pub async fn apply(payload: &GltfReorderNodeChildrenPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.nodes[payload.parent].children = payload.order.clone(); Ok(next) }
