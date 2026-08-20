//! 🔺️ change-asset-extra-data direct sparse-diff derivation.
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::diff::{GltfDiff, GltfAssetDiff};
use crate::artifacts::gltf::schema::mutations::change_asset_extra_data::mutation::{apply, GltfChangeAssetExtraDataPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfChangeAssetExtraDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let _ = apply(payload, base)?; Ok(GltfDiff { asset: Some(GltfAssetDiff { extras: Some(payload.data.clone()), ..Default::default() }), ..Default::default() }) }
