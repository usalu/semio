//! 🔺️ move-required-extension direct sparse-diff derivation.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::move_required_extension::mutation::{apply, GltfMoveRequiredExtensionPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfMoveRequiredExtensionPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { extensions_required: Some({ let mut values = base.document.extensions_required.clone(); let value = values.remove(values.iter().position(|value| value == &payload.extension).unwrap()); values.insert(payload.position, value); values }), ..Default::default() }) }
