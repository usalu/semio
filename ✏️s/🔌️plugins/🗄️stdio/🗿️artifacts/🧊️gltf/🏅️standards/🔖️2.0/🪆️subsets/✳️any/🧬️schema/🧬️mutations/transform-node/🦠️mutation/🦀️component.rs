//! 🦀 transform-node: typed validation and atomic application.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::mutations::top_level_private::{GltfTopLevelMutationRejection, reject};
use crate::artifacts::gltf::schema::mutations::structure_geometry_private::{checked_index, checked_position};
pub const ID: &str = "s.stdio.gltf.mutation.transform-node.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfTransformNodePayload { pub node: usize, pub transform: GltfNodeTransform }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfNodeTransform { Matrix { matrix: [f64; 16] }, Trs { translation: Option<[f64; 3]>, rotation: Option<[f64; 4]>, scale: Option<[f64; 3]> } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfTransformNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { checked_index(payload.node, base.document.nodes.len(), "document/nodes")?; let finite = match &payload.transform { GltfNodeTransform::Matrix { matrix } => matrix.iter().all(|value| value.is_finite()), GltfNodeTransform::Trs { translation, rotation, scale } => translation.iter().flatten().chain(rotation.iter().flatten()).chain(scale.iter().flatten()).all(|value| value.is_finite()) }; if !finite { return Err(reject("gltf.mutation.invalid-transform", format!("document/nodes/{}/transform", payload.node), "transform values must be finite")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfTransformNodePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); let node = &mut next.document.nodes[payload.node]; match &payload.transform { GltfNodeTransform::Matrix { matrix } => { node.matrix = Some(*matrix); node.translation = None; node.rotation = None; node.scale = None; }, GltfNodeTransform::Trs { translation, rotation, scale } => { node.matrix = None; node.translation = *translation; node.rotation = *rotation; node.scale = *scale; } } Ok(next) }
