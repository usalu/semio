//! 🦀 delete-morph-target: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.delete-morph-target.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfDeleteMorphTargetPayload { pub mesh: usize, pub primitive: usize, pub target: usize }
pub fn validate(payload: &GltfDeleteMorphTargetPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?; if base.document.meshes[payload.mesh].primitives.len() != 1 || !base.document.meshes[payload.mesh].weights.is_empty() { return Err(reject("gltf.mutation.morph-target-arity", "document/meshes", "target deletion would violate mesh target-count coherence")); } Ok(()) }
pub fn apply(payload: &GltfDeleteMorphTargetPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.meshes[payload.mesh].primitives[payload.primitive].targets.remove(payload.target); Ok(next) }
