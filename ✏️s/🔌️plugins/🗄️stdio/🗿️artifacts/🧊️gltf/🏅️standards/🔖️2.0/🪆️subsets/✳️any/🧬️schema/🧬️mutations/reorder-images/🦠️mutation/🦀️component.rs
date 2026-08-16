//! 🦠️ reorder-images typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-images.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/images"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderImagesPayload { pub order: Vec<usize> }
pub fn validate(payload: &GltfReorderImagesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.images.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.images.len()) { return Err(reject("gltf.mutation.invalid-permutation", "document/images", "order must contain every index once")); } if payload.order.iter().enumerate().all(|(index, value)| index == *value) { return Err(reject("gltf.mutation.no-observable-change", "document/images", "order already matches")); }  Ok(()) }
pub fn apply(payload: &GltfReorderImagesPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); images_op(&mut next, GltfTopLevelFamily::Images, payload.order[0], None, Some(&payload.order))?;  Ok(next) }
