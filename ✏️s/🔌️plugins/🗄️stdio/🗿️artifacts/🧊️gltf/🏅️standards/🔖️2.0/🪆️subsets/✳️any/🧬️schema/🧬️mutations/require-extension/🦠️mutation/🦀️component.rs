//! 🦠️ require-extension executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.require-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfRequireExtensionPayload { pub extension: String, pub position: usize }
pub async fn validate(payload: &GltfRequireExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.extension.trim().is_empty() { return Err(reject("gltf.mutation.invalid-extension", "document/extensionsRequired", "extension must be non-empty")); } if base.document.extensions_required.contains(&payload.extension) { return Err(reject("gltf.mutation.duplicate-extension", "document/extensionsRequired", "extension is already declared")); } if !base.document.extensions_used.contains(&payload.extension) { return Err(reject("gltf.mutation.required-extension-not-used", "document/extensionsRequired", "a required extension must first be used")); } if payload.position > base.document.extensions_required.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/extensionsRequired", "position must be within the declaration list")); } Ok(()) }
pub async fn apply(payload: &GltfRequireExtensionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions_required.insert(payload.position, payload.extension.clone()); Ok(next) }
