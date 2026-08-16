//! 🔺️ change-asset-extension-data direct sparse-diff derivation.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::change_asset_extension_data::mutation::{apply, GltfChangeAssetExtensionDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub fn derive(payload: &GltfChangeAssetExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { asset: Some(GltfAssetDiff { extensions: Some(payload.data.clone()), ..Default::default() }), ..Default::default() }) }
