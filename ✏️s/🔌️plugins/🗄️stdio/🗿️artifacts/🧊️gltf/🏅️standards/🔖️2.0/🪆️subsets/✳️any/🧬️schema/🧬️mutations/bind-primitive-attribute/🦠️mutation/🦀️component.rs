//! 🦀 bind-primitive-attribute: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.bind-primitive-attribute.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfBindPrimitiveAttributePayload { pub mesh: usize, pub primitive: usize, pub semantic: String, pub accessor: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindPrimitiveAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; checked_index(payload.accessor, base.document.accessors.len(), "document/accessors")?; if payload.semantic.trim().is_empty() || base.document.meshes[payload.mesh].primitives[payload.primitive].attributes.iter().any(|(semantic, _)| semantic == &payload.semantic) { return Err(reject("gltf.mutation.invalid-attribute-semantic", "document/meshes/primitives/attributes", "semantic must be non-empty and unique")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfBindPrimitiveAttributePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.meshes[payload.mesh].primitives[payload.primitive].attributes.push((payload.semantic.clone(), payload.accessor)); Ok(next) }
