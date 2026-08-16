//! 🔺️ change-asset-descriptive-metadata direct sparse-diff derivation.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::change_asset_descriptive_metadata::mutation::{apply, GltfChangeAssetDescriptiveMetadataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub fn derive(payload: &GltfChangeAssetDescriptiveMetadataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { asset: Some(GltfAssetDiff { generator: Some(payload.generator.clone()), copyright: Some(payload.copyright.clone()), min_version: Some(payload.min_version.clone()), ..Default::default() }), ..Default::default() }) }
