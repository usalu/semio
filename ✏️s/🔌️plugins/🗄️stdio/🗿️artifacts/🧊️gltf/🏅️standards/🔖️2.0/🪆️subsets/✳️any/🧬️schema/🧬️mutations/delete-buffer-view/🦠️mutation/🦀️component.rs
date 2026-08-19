//! 🦠️ delete-buffer-view typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.delete-buffer-view.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/bufferViews"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteBufferViewPayload { pub index: usize }
pub async fn validate(payload: &GltfDeleteBufferViewPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.buffer_views.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/bufferViews", "index must address an item")); }  Ok(()) }
pub async fn apply(payload: &GltfDeleteBufferViewPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); buffer_views_op(&mut next, GltfTopLevelFamily::BufferViews, payload.index, None, None)?;  Ok(next) }
