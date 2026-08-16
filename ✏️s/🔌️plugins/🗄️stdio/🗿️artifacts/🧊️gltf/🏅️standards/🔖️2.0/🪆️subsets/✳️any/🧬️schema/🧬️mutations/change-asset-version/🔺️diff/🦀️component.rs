//! 🔺️ change-asset-version direct sparse-diff derivation.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::change_asset_version::mutation::{apply, GltfChangeAssetVersionPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub fn derive(payload: &GltfChangeAssetVersionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { asset: Some(GltfAssetDiff { version: Some(payload.version.clone()), ..Default::default() }), ..Default::default() }) }
