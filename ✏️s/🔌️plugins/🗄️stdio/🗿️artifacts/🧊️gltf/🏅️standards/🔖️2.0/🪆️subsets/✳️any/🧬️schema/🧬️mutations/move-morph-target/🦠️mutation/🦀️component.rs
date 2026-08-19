//! 🦀 move-morph-target: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.move-morph-target.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMoveMorphTargetPayload { pub mesh: usize, pub primitive: usize, pub target: usize, pub position: usize }
pub async fn validate(payload: &GltfMoveMorphTargetPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?; checked_index(payload.position, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?; if payload.target == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives/targets", "destination equals source")); } Ok(()) }
pub async fn apply(payload: &GltfMoveMorphTargetPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let target = next.document.meshes[payload.mesh].primitives[payload.primitive].targets.remove(payload.target); next.document.meshes[payload.mesh].primitives[payload.primitive].targets.insert(payload.position, target); Ok(next) }
