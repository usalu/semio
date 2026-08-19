//! 🦠️ delete-sampler typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.delete-sampler.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/samplers"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteSamplerPayload { pub index: usize }
pub async fn validate(payload: &GltfDeleteSamplerPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.samplers.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/samplers", "index must address an item")); }  Ok(()) }
pub async fn apply(payload: &GltfDeleteSamplerPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); samplers_op(&mut next, GltfTopLevelFamily::Samplers, payload.index, None, None)?;  Ok(next) }
