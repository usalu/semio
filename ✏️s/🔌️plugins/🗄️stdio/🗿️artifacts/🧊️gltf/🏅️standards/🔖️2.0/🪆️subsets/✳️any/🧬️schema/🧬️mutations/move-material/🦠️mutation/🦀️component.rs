//! 🦠️ move-material typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-material.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveMaterialPayload { pub index: usize, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveMaterialPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.materials.len() || payload.position >= base.document.materials.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/materials", "indices must address items")); } if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/materials", "destination equals source")); }  Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveMaterialPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); materials_op(&mut next, GltfTopLevelFamily::Materials, payload.index, Some(payload.position), None)?;  Ok(next) }
