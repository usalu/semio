//! 🦠️ move-texture typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-texture.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/textures"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveTexturePayload { pub index: usize, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveTexturePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.textures.len() || payload.position >= base.document.textures.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/textures", "indices must address items")); } if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/textures", "destination equals source")); }  Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveTexturePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); textures_op(&mut next, GltfTopLevelFamily::Textures, payload.index, Some(payload.position), None)?;  Ok(next) }
