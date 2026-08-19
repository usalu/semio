//! 🦠️ reorder-used-extensions executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-used-extensions.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsUsed"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfReorderUsedExtensionsPayload { pub order: Vec<String> }
pub async fn validate(payload: &GltfReorderUsedExtensionsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.extensions_used.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>() .len() != payload.order.len() || payload.order.iter().any(|value| !base.document.extensions_used.contains(value)) { return Err(reject("gltf.mutation.invalid-permutation", "document/extensionsUsed", "order must contain every declaration exactly once")); } if payload.order == base.document.extensions_used { return Err(reject("gltf.mutation.no-observable-change", "document/extensionsUsed", "order already matches")); } Ok(()) }
pub async fn apply(payload: &GltfReorderUsedExtensionsPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions_used = payload.order.clone(); Ok(next) }
