//! 🦀 reorder-scene-root-nodes: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-scene-root-nodes.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfReorderSceneRootNodesPayload { pub scene: usize, pub order: Vec<usize> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderSceneRootNodesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?; let roots = &base.document.scenes[payload.scene].nodes; if payload.order.len() != roots.len() || payload.order.iter().any(|node| !roots.contains(node)) || { let mut order = payload.order.clone(); order.sort_unstable(); order.dedup(); order.len() != roots.len() } { return Err(reject("gltf.mutation.invalid-permutation", "document/scenes/nodes", "order must contain every root identity once")); } if payload.order == *roots { return Err(reject("gltf.mutation.no-observable-change", "document/scenes/nodes", "reorder must change order")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfReorderSceneRootNodesPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.scenes[payload.scene].nodes = payload.order.clone(); Ok(next) }
