//! 🦠️ reorder-required-extensions executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-required-extensions.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfReorderRequiredExtensionsPayload { pub order: Vec<String> }
pub async fn validate(payload: &GltfReorderRequiredExtensionsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.order.len() != base.document.extensions_required.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>() .len() != payload.order.len() || payload.order.iter().any(|value| !base.document.extensions_required.contains(value)) { return Err(reject("gltf.mutation.invalid-permutation", "document/extensionsRequired", "order must contain every declaration exactly once")); } if payload.order == base.document.extensions_required { return Err(reject("gltf.mutation.no-observable-change", "document/extensionsRequired", "order already matches")); } Ok(()) }
pub async fn apply(payload: &GltfReorderRequiredExtensionsPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions_required = payload.order.clone(); Ok(next) }
