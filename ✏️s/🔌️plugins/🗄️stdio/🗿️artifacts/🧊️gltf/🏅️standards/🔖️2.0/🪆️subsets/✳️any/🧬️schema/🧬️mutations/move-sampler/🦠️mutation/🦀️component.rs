//! 🦠️ move-sampler typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-sampler.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/samplers"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveSamplerPayload { pub index: usize, pub position: usize }
pub fn validate(payload: &GltfMoveSamplerPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.samplers.len() || payload.position >= base.document.samplers.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/samplers", "indices must address items")); } if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/samplers", "destination equals source")); }  Ok(()) }
pub fn apply(payload: &GltfMoveSamplerPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); samplers_op(&mut next, GltfTopLevelFamily::Samplers, payload.index, Some(payload.position), None)?;  Ok(next) }
