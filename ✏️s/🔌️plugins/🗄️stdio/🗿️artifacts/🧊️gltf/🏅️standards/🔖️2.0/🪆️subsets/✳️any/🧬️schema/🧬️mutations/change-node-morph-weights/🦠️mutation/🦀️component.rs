//! 🦀 change-node-morph-weights: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.change-node-morph-weights.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeNodeMorphWeightsPayload { pub node: usize, pub weights: Vec<f64> }
pub fn validate(payload: &GltfChangeNodeMorphWeightsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.node, base.document.nodes.len(), "document/nodes")?; if !payload.weights.iter().all(|value| value.is_finite()) { return Err(reject("gltf.mutation.invalid-morph-weights", "document/nodes/weights", "weights must be finite")); } let mesh = base.document.nodes[payload.node].mesh; if !payload.weights.is_empty() && mesh.is_none() { return Err(reject("gltf.mutation.missing-mesh", "document/nodes/mesh", "morph weights require a mesh")); } if let Some(mesh) = mesh { if base.document.meshes[mesh].primitives.iter().any(|primitive| primitive.targets.len() != payload.weights.len()) { return Err(reject("gltf.mutation.morph-weight-arity", "document/nodes/weights", "weights must match primitive target count")); } } Ok(()) }
pub fn apply(payload: &GltfChangeNodeMorphWeightsPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.nodes[payload.node].weights = payload.weights.clone(); Ok(next) }
