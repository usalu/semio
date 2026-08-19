//! 🔺️ change-node-morph-weights direct typed field diff.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_node_morph_weights::mutation::{validate,GltfChangeNodeMorphWeightsPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeNodeMorphWeightsDiff{pub operation:GltfChangeNodeMorphWeightsPayload,pub after:Vec<f64>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfChangeNodeMorphWeightsPayload,base:&GltfSnapshot)->Result<GltfChangeNodeMorphWeightsDiff,GltfTopLevelMutationRejection>{validate(operation,base)?;let after=operation.weights.clone();Ok(GltfChangeNodeMorphWeightsDiff{operation:operation.clone(),after,touched_paths:["document/nodes/*/weights"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,diff:&GltfChangeNodeMorphWeightsDiff)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].weights=diff.after.clone();Ok(next)}
pub async fn encode(diff:&GltfChangeNodeMorphWeightsDiff)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(diff)}
