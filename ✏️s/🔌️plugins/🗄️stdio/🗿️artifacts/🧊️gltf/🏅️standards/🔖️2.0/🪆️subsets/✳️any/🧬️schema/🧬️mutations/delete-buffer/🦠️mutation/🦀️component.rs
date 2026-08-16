//! 🦠️ delete-buffer typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.delete-buffer.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/buffers"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteBufferPayload { pub index: usize }
pub fn validate(payload: &GltfDeleteBufferPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.buffers.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/buffers", "index must address an item")); } if base.document.buffers.len() != base.buffers.len() { return Err(reject("gltf.mutation.buffer-alignment", "buffers", "descriptor and bytes arrays must align")); } Ok(()) }
pub fn apply(payload: &GltfDeleteBufferPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); buffers_op(&mut next, GltfTopLevelFamily::Buffers, payload.index, None, None)?; next.buffers.remove(payload.index); Ok(next) }
