//! 🦠️ change-asset-descriptive-metadata executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-descriptive-metadata.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/generator", "document/asset/copyright", "document/asset/minVersion"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeAssetDescriptiveMetadataPayload { pub generator: Option<String>, pub copyright: Option<String>, pub min_version: Option<String> }
pub async fn validate(payload: &GltfChangeAssetDescriptiveMetadataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.generator == base.document.asset.generator && payload.copyright == base.document.asset.copyright && payload.min_version == base.document.asset.min_version { return Err(reject("gltf.mutation.no-observable-change", "document/asset", "descriptive metadata already has these values")); } Ok(()) }
pub async fn apply(payload: &GltfChangeAssetDescriptiveMetadataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.asset.generator = payload.generator.clone(); next.document.asset.copyright = payload.copyright.clone(); next.document.asset.min_version = payload.min_version.clone(); Ok(next) }
