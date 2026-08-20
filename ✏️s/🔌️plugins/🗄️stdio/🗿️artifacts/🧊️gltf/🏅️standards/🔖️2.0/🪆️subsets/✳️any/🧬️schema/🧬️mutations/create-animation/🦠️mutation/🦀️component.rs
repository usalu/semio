//! 🦠️ create-animation typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-animation.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/animations"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateAnimationPayload { pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateAnimationPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.animations.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/animations", "position must be within the collection")); }   Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfCreateAnimationPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Animations, &Change::Insert(payload.position))?; next.document.animations.insert(payload.position, GltfAnimation::default()); Ok(next) }
