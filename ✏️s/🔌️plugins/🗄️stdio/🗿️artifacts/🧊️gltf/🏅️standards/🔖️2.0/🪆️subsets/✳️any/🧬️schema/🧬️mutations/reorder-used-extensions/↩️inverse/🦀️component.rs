//! ↩️ reorder-used-extensions inverse derived from exact base values.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::reorder_used_extensions::mutation::{apply, GltfReorderUsedExtensionsPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfReorderUsedExtensionsPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { extensions_used: Some(base.document.extensions_used.clone()), ..Default::default() }) }
