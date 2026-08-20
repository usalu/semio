//! 🦠️ create-buffer typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-buffer.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/buffers"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateBufferPayload { pub position: usize, pub bytes: Vec<u8> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateBufferPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.buffers.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/buffers", "position must be within the collection")); }  if base.document.buffers.len() != base.buffers.len() { return Err(reject("gltf.mutation.buffer-alignment", "buffers", "descriptor and bytes arrays must align")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfCreateBufferPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Buffers, &Change::Insert(payload.position))?; next.document.buffers.insert(payload.position, GltfBuffer { byte_length: payload.bytes.len(), uri: None, name: None, extensions: None, extras: None }); next.buffers.insert(payload.position, payload.bytes.clone()); Ok(next) }
