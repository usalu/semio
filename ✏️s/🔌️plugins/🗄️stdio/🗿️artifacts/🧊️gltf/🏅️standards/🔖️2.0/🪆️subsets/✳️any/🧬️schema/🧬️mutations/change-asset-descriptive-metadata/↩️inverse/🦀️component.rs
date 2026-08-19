//! ↩️ change-asset-descriptive-metadata inverse derived from exact base values.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::change_asset_descriptive_metadata::mutation::{apply, GltfChangeAssetDescriptiveMetadataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfChangeAssetDescriptiveMetadataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { asset: Some(GltfAssetDiff { generator: Some(base.document.asset.generator.clone()), copyright: Some(base.document.asset.copyright.clone()), min_version: Some(base.document.asset.min_version.clone()), ..Default::default() }), ..Default::default() }) }
