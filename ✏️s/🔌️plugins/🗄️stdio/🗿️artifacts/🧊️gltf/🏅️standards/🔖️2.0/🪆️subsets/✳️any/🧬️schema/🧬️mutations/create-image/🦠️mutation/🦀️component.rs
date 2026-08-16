//! 🦠️ create-image typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-image.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/images"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateImagePayload { pub position: usize }
pub fn validate(payload: &GltfCreateImagePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.images.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/images", "position must be within the collection")); }   Ok(()) }
pub fn apply(payload: &GltfCreateImagePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Images, &Change::Insert(payload.position))?; next.document.images.insert(payload.position, GltfImage::default()); Ok(next) }
