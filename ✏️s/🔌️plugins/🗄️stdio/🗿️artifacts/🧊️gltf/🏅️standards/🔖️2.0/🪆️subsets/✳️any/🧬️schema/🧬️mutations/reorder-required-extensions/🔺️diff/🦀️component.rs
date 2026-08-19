//! 🔺️ reorder-required-extensions direct sparse-diff derivation.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::reorder_required_extensions::mutation::{apply, GltfReorderRequiredExtensionsPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfReorderRequiredExtensionsPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { extensions_required: Some(payload.order.clone()), ..Default::default() }) }
