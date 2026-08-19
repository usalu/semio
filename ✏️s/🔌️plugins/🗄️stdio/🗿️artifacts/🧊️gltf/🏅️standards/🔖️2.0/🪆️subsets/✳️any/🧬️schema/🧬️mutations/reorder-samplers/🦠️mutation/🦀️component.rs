//! 🦠️ reorder-samplers typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-samplers.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/samplers"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderSamplersPayload { pub order: Vec<usize> }
pub async fn validate(payload: &GltfReorderSamplersPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.samplers.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.samplers.len()) { return Err(reject("gltf.mutation.invalid-permutation", "document/samplers", "order must contain every index once")); } if payload.order.iter().enumerate().all(|(index, value)| index == *value) { return Err(reject("gltf.mutation.no-observable-change", "document/samplers", "order already matches")); }  Ok(()) }
pub async fn apply(payload: &GltfReorderSamplersPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); samplers_op(&mut next, GltfTopLevelFamily::Samplers, payload.order[0], None, Some(&payload.order))?;  Ok(next) }
