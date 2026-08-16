//! 🦠️ create-sampler typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-sampler.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/samplers"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateSamplerPayload { pub position: usize }
pub fn validate(payload: &GltfCreateSamplerPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.samplers.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/samplers", "position must be within the collection")); }   Ok(()) }
pub fn apply(payload: &GltfCreateSamplerPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Samplers, &Change::Insert(payload.position))?; next.document.samplers.insert(payload.position, GltfSampler::default()); Ok(next) }
