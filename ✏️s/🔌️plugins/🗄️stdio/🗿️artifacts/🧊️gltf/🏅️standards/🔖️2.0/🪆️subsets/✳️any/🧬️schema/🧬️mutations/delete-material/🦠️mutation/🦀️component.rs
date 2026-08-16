//! 🦠️ delete-material typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.delete-material.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteMaterialPayload { pub index: usize }
pub fn validate(payload: &GltfDeleteMaterialPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.materials.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/materials", "index must address an item")); }  Ok(()) }
pub fn apply(payload: &GltfDeleteMaterialPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); materials_op(&mut next, GltfTopLevelFamily::Materials, payload.index, None, None)?;  Ok(next) }
