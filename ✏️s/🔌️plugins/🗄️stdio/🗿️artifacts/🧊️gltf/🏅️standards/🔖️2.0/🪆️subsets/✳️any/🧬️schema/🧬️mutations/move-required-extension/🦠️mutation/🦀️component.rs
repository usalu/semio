//! 🦠️ move-required-extension executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.move-required-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMoveRequiredExtensionPayload { pub extension: String, pub position: usize }
pub fn validate(payload: &GltfMoveRequiredExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { let index = base.document.extensions_required.iter().position(|value| value == &payload.extension).ok_or_else(|| reject("gltf.mutation.extension-absent", "document/extensionsRequired", "extension is not declared"))?; if payload.position >= base.document.extensions_required.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/extensionsRequired", "position must address a declaration")); } if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/extensionsRequired", "destination equals source")); } Ok(()) }
pub fn apply(payload: &GltfMoveRequiredExtensionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let value = next.document.extensions_required.remove(next.document.extensions_required.iter().position(|value| value == &payload.extension).unwrap()); next.document.extensions_required.insert(payload.position, value); Ok(next) }
