//! 🦀 unbind-primitive-material: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.unbind-primitive-material.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfUnbindPrimitiveMaterialPayload { pub mesh: usize, pub primitive: usize }
pub fn validate(payload: &GltfUnbindPrimitiveMaterialPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; if base.document.meshes[payload.mesh].primitives[payload.primitive].material.is_none() { return Err(reject("gltf.mutation.relation-absent", "document/meshes/primitives/material", "primitive has no material")); } Ok(()) }
pub fn apply(payload: &GltfUnbindPrimitiveMaterialPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.meshes[payload.mesh].primitives[payload.primitive].material = None; Ok(next) }
