//! 🦠️ create-buffer-view typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-buffer-view.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/bufferViews"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateBufferViewPayload { pub position: usize, pub buffer: usize, pub byte_offset: usize, pub byte_length: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateBufferViewPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.buffer_views.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/bufferViews", "position must be within the collection")); } if payload.buffer >= base.document.buffers.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/buffers", "backing buffer must exist")); }  Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfCreateBufferViewPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::BufferViews, &Change::Insert(payload.position))?; next.document.buffer_views.insert(payload.position, GltfBufferView { buffer: payload.buffer, byte_offset: payload.byte_offset, byte_length: payload.byte_length, byte_stride: None, target: None, name: None, extensions: None, extras: None }); Ok(next) }
