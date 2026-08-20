//! 🦀 reorder-primitive-attributes: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-primitive-attributes.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfReorderPrimitiveAttributesPayload { pub mesh: usize, pub primitive: usize, pub order: Vec<String> }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderPrimitiveAttributesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; let attributes = &base.document.meshes[payload.mesh].primitives[payload.primitive].attributes; if payload.order.len() != attributes.len() || payload.order.iter().any(|semantic| !attributes.iter().any(|(key, _)| key == semantic)) || { let mut order = payload.order.clone(); order.sort(); order.dedup(); order.len() != attributes.len() } { return Err(reject("gltf.mutation.invalid-permutation", "document/meshes/primitives/attributes", "order must contain every semantic once")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfReorderPrimitiveAttributesPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let prior = next.document.meshes[payload.mesh].primitives[payload.primitive].attributes.clone(); next.document.meshes[payload.mesh].primitives[payload.primitive].attributes = payload.order.iter().map(|semantic| prior.iter().find(|(key, _)| key == semantic).expect("validated semantic").clone()).collect(); Ok(next) }
