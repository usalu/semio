//! 🦠️ move-accessor typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-accessor.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/accessors"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveAccessorPayload { pub index: usize, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveAccessorPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.accessors.len() || payload.position >= base.document.accessors.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/accessors", "indices must address items")); } if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/accessors", "destination equals source")); }  Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveAccessorPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); accessors_op(&mut next, GltfTopLevelFamily::Accessors, payload.index, Some(payload.position), None)?;  Ok(next) }
