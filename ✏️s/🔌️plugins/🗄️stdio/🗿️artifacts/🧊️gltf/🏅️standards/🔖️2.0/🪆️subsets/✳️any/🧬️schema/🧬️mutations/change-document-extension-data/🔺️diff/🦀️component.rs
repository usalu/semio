//! 🔺️ change-document-extension-data direct sparse-diff derivation.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::change_document_extension_data::mutation::{apply, GltfChangeDocumentExtensionDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub async fn derive(payload: &GltfChangeDocumentExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { extensions: Some(payload.data.clone()), ..Default::default() }) }
