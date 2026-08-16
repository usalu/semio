//! 🦀 move-morph-target-attribute: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.move-morph-target-attribute.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfMoveMorphTargetAttributePayload { pub mesh: usize, pub primitive: usize, pub target: usize, pub semantic: String, pub position: usize }
pub fn validate(payload: &GltfMoveMorphTargetAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?; checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?; let attributes = &base.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0; let index = attributes.iter().position(|(semantic, _)| semantic == &payload.semantic).ok_or_else(|| reject("gltf.mutation.relation-absent", "document/meshes/primitives/targets", "semantic is not bound"))?; checked_index(payload.position, attributes.len(), "document/meshes/primitives/targets")?; if index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives/targets", "destination equals source")); } Ok(()) }
pub fn apply(payload: &GltfMoveMorphTargetAttributePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let attributes = &mut next.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0; let index = attributes.iter().position(|(semantic, _)| semantic == &payload.semantic).expect("validated semantic"); let entry = attributes.remove(index); attributes.insert(payload.position, entry); Ok(next) }
