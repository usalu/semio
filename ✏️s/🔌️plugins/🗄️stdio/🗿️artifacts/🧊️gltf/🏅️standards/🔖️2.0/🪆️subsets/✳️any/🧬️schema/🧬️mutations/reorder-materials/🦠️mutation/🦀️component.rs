//! 🦠️ reorder-materials typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-materials.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderMaterialsPayload { pub order: Vec<usize> }
pub async fn validate(payload: &GltfReorderMaterialsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.materials.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.materials.len()) { return Err(reject("gltf.mutation.invalid-permutation", "document/materials", "order must contain every index once")); } if payload.order.iter().enumerate().all(|(index, value)| index == *value) { return Err(reject("gltf.mutation.no-observable-change", "document/materials", "order already matches")); }  Ok(()) }
pub async fn apply(payload: &GltfReorderMaterialsPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); materials_op(&mut next, GltfTopLevelFamily::Materials, payload.order[0], None, Some(&payload.order))?;  Ok(next) }
