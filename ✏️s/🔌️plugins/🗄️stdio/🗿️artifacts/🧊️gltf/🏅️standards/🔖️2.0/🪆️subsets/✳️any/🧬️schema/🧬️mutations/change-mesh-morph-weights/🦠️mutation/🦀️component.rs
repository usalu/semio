//! 🦀 change-mesh-morph-weights: typed, validated, atomic Rust facet.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.change-mesh-morph-weights.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMeshMorphWeightsPayload { pub mesh: usize, pub weights: Vec<f64> }
pub async fn validate(payload: &GltfChangeMeshMorphWeightsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?; if !payload.weights.iter().all(|value| value.is_finite()) || base.document.meshes[payload.mesh].primitives.iter().any(|primitive| primitive.targets.len() != payload.weights.len()) { return Err(reject("gltf.mutation.invalid-morph-weights", "document/meshes/weights", "weights must be finite and match every primitive target list")); } Ok(()) }
pub async fn apply(payload: &GltfChangeMeshMorphWeightsPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.meshes[payload.mesh].weights = payload.weights.clone(); Ok(next) }
