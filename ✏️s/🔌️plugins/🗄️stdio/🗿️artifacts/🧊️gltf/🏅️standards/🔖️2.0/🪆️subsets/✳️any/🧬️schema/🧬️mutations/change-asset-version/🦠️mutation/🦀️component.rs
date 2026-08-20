//! 🦠️ change-asset-version executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-version.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/version"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeAssetVersionPayload { pub version: String }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeAssetVersionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.version.trim().is_empty() { return Err(reject("gltf.mutation.invalid-asset-version", "document/asset/version", "version must be non-empty")); } if payload.version == base.document.asset.version { return Err(reject("gltf.mutation.no-observable-change", "document/asset/version", "version already has this value")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfChangeAssetVersionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.asset.version = payload.version.clone(); Ok(next) }
