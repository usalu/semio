//! 🦠️ create-material typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-material.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateMaterialPayload { pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateMaterialPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.materials.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/materials", "position must be within the collection")); }   Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfCreateMaterialPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Materials, &Change::Insert(payload.position))?; next.document.materials.insert(payload.position, GltfMaterial::default()); Ok(next) }
