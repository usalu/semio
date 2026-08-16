//! 🦀 reorder-primitives: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-primitives.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfReorderPrimitivesPayload { pub mesh: usize, pub order: Vec<usize> }
pub fn validate(payload: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; let length = base.document.meshes[payload.mesh].primitives.len(); if payload.order.len() != length || payload.order.iter().any(|index| *index >= length) || { let mut order = payload.order.clone(); order.sort_unstable(); order.dedup(); order.len() != length } { return Err(reject("gltf.mutation.invalid-permutation", "document/meshes/primitives", "order must contain each primitive once")); } if payload.order.iter().enumerate().all(|(index, value)| *value == index) { return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives", "reorder must change order")); } Ok(()) }
pub fn apply(payload: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let prior = next.document.meshes[payload.mesh].primitives.clone(); next.document.meshes[payload.mesh].primitives = payload.order.iter().map(|index| prior[*index].clone()).collect(); Ok(next) }
