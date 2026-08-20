//! 🦀 reorder-morph-target-attributes: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-morph-target-attributes.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfReorderMorphTargetAttributesPayload { pub mesh: usize, pub primitive: usize, pub target: usize, pub order: Vec<String> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderMorphTargetAttributesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?; let attributes = &base.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0; if payload.order.len() != attributes.len() || payload.order.iter().any(|semantic| !attributes.iter().any(|(key, _)| key == semantic)) { return Err(reject("gltf.mutation.invalid-permutation", "document/meshes/primitives/targets", "order must contain every semantic once")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfReorderMorphTargetAttributesPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let prior = next.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0.clone(); next.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0 = payload.order.iter().map(|semantic| prior.iter().find(|(key, _)| key == semantic).expect("validated semantic").clone()).collect(); Ok(next) }
