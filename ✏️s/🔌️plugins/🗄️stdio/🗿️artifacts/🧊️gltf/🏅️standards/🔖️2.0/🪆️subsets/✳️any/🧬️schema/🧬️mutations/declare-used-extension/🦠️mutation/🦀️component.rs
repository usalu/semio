//! 🦠️ declare-used-extension executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.declare-used-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsUsed"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfDeclareUsedExtensionPayload { pub extension: String, pub position: usize }
pub async fn validate(payload: &GltfDeclareUsedExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.extension.trim().is_empty() { return Err(reject("gltf.mutation.invalid-extension", "document/extensionsUsed", "extension must be non-empty")); } if base.document.extensions_used.contains(&payload.extension) { return Err(reject("gltf.mutation.duplicate-extension", "document/extensionsUsed", "extension is already declared")); }  if payload.position > base.document.extensions_used.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/extensionsUsed", "position must be within the declaration list")); } Ok(()) }
pub async fn apply(payload: &GltfDeclareUsedExtensionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions_used.insert(payload.position, payload.extension.clone()); Ok(next) }
