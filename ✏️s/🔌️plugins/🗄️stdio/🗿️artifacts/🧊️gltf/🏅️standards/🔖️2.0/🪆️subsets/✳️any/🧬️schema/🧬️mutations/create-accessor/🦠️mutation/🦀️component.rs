//! 🦠️ create-accessor typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-accessor.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/accessors"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateAccessorPayload { pub position: usize, pub component_type: GltfComponentType, pub count: usize, pub kind: GltfAccessorType }
pub async fn validate(payload: &GltfCreateAccessorPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.accessors.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/accessors", "position must be within the collection")); }   Ok(()) }
pub async fn apply(payload: &GltfCreateAccessorPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Accessors, &Change::Insert(payload.position))?; next.document.accessors.insert(payload.position, GltfAccessor { buffer_view: None, byte_offset: 0, component_type: payload.component_type, normalized: false, count: payload.count, kind: payload.kind, max: None, min: None, sparse: None, name: None, extensions: None, extras: None }); Ok(next) }
