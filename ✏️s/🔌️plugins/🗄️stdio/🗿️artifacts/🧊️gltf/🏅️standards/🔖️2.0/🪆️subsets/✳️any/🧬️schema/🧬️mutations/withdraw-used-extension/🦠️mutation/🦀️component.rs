//! 🦠️ withdraw-used-extension executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.withdraw-used-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsUsed"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfWithdrawUsedExtensionPayload { pub extension: String }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfWithdrawUsedExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if !base.document.extensions_used.contains(&payload.extension) { return Err(reject("gltf.mutation.extension-absent", "document/extensionsUsed", "extension is not declared")); } if base.document.extensions_required.contains(&payload.extension) { return Err(reject("gltf.mutation.extension-required", "document/extensionsRequired", "remove the requirement first")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfWithdrawUsedExtensionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions_used.retain(|value| value != &payload.extension); Ok(next) }
