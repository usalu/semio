//! 🦀 move-primitive-attribute: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.move-primitive-attribute.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMovePrimitiveAttributePayload { pub mesh: usize, pub primitive: usize, pub semantic: String, pub position: usize }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMovePrimitiveAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; let attributes = &base.document.meshes[payload.mesh].primitives[payload.primitive].attributes; let index = attributes.iter().position(|(semantic, _)| semantic == &payload.semantic).ok_or_else(|| reject("gltf.mutation.relation-absent", "document/meshes/primitives/attributes", "semantic is not bound"))?; checked_index(payload.position, attributes.len(), "document/meshes/primitives/attributes")?; if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives/attributes", "destination equals source")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfMovePrimitiveAttributePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let attributes = &mut next.document.meshes[payload.mesh].primitives[payload.primitive].attributes; let index = attributes.iter().position(|(semantic, _)| semantic == &payload.semantic).expect("validated semantic"); let attribute = attributes.remove(index); attributes.insert(payload.position, attribute); Ok(next) }
