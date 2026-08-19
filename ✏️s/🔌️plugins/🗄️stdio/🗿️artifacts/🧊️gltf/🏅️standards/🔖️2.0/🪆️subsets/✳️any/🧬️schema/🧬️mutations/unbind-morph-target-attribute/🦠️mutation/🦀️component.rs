//! 🦀 unbind-morph-target-attribute: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.unbind-morph-target-attribute.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfUnbindMorphTargetAttributePayload { pub mesh: usize, pub primitive: usize, pub target: usize, pub semantic: String }
pub async fn validate(payload: &GltfUnbindMorphTargetAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?; if !base.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0.iter().any(|(semantic, _)| semantic == &payload.semantic) { return Err(reject("gltf.mutation.relation-absent", "document/meshes/primitives/targets", "semantic is not bound")); } Ok(()) }
pub async fn apply(payload: &GltfUnbindMorphTargetAttributePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0.retain(|(semantic, _)| semantic != &payload.semantic); Ok(next) }
