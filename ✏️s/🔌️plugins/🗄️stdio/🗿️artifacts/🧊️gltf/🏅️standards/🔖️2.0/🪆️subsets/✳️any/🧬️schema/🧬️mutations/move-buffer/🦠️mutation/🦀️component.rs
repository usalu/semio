//! 🦠️ move-buffer typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-buffer.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/buffers"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveBufferPayload { pub index: usize, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveBufferPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.buffers.len() || payload.position >= base.document.buffers.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/buffers", "indices must address items")); } if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/buffers", "destination equals source")); } if base.document.buffers.len() != base.buffers.len() { return Err(reject("gltf.mutation.buffer-alignment", "buffers", "descriptor and bytes arrays must align")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveBufferPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); buffers_op(&mut next, GltfTopLevelFamily::Buffers, payload.index, Some(payload.position), None)?; let bytes = next.buffers.remove(payload.index); next.buffers.insert(payload.position, bytes); Ok(next) }
