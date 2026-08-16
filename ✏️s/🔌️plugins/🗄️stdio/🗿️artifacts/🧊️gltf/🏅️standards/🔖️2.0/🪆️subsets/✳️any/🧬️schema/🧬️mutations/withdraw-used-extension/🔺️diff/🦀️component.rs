//! 🔺️ withdraw-used-extension direct sparse-diff derivation.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::withdraw_used_extension::mutation::{apply, GltfWithdrawUsedExtensionPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub fn derive(payload: &GltfWithdrawUsedExtensionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { extensions_used: Some(base.document.extensions_used.iter().filter(|value| *value != &payload.extension).cloned().collect()), ..Default::default() }) }
