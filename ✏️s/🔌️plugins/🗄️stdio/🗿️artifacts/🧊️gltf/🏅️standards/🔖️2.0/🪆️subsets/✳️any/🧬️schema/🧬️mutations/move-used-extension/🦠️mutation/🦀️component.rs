//! 🦠️ move-used-extension executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.move-used-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsUsed"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMoveUsedExtensionPayload { pub extension: String, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveUsedExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { let index = base.document.extensions_used.iter().position(|value| value == &payload.extension).ok_or_else(|| reject("gltf.mutation.extension-absent", "document/extensionsUsed", "extension is not declared"))?; if payload.position >= base.document.extensions_used.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/extensionsUsed", "position must address a declaration")); } if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/extensionsUsed", "destination equals source")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMoveUsedExtensionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let value = next.document.extensions_used.remove(next.document.extensions_used.iter().position(|value| value == &payload.extension).unwrap()); next.document.extensions_used.insert(payload.position, value); Ok(next) }
