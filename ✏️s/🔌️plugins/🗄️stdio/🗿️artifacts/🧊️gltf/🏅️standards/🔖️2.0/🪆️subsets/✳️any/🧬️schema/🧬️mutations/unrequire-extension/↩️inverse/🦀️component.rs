//! ↩️ unrequire-extension inverse derived from exact base values.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::unrequire_extension::mutation::{apply, GltfUnrequireExtensionPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
pub fn derive(payload: &GltfUnrequireExtensionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { extensions_required: Some(base.document.extensions_required.clone()), ..Default::default() }) }
