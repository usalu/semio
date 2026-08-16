//! 🦠️ reorder-buffers typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-buffers.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/buffers"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderBuffersPayload { pub order: Vec<usize> }
pub fn validate(payload: &GltfReorderBuffersPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.buffers.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.buffers.len()) { return Err(reject("gltf.mutation.invalid-permutation", "document/buffers", "order must contain every index once")); } if payload.order.iter().enumerate().all(|(index, value)| index == *value) { return Err(reject("gltf.mutation.no-observable-change", "document/buffers", "order already matches")); } if base.document.buffers.len() != base.buffers.len() { return Err(reject("gltf.mutation.buffer-alignment", "buffers", "descriptor and bytes arrays must align")); } Ok(()) }
pub fn apply(payload: &GltfReorderBuffersPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); buffers_op(&mut next, GltfTopLevelFamily::Buffers, payload.order[0], None, Some(&payload.order))?; next.buffers = payload.order.iter().map(|index| next.buffers[*index].clone()).collect(); Ok(next) }
