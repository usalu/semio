//! 🦠️ reorder-textures typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-textures.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/textures"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderTexturesPayload { pub order: Vec<usize> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderTexturesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.textures.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.textures.len()) { return Err(reject("gltf.mutation.invalid-permutation", "document/textures", "order must contain every index once")); } if payload.order.iter().enumerate().all(|(index, value)| index == *value) { return Err(reject("gltf.mutation.no-observable-change", "document/textures", "order already matches")); }  Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfReorderTexturesPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); textures_op(&mut next, GltfTopLevelFamily::Textures, payload.order[0], None, Some(&payload.order))?;  Ok(next) }
