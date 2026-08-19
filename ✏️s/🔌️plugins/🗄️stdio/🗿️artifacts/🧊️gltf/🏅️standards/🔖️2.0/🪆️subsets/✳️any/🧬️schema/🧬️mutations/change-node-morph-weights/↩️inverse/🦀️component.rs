//! ↩️ change-node-morph-weights exact typed field inverse.
use serde::{Deserialize,Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::schema::mutations::change_node_morph_weights::mutation::{validate,GltfChangeNodeMorphWeightsPayload};
use crate::artifacts::gltf::schema::mutations::top_level_private::GltfTopLevelMutationRejection;
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]#[serde(rename_all="camelCase")]
pub struct GltfChangeNodeMorphWeightsInverse{pub operation:GltfChangeNodeMorphWeightsPayload,pub before:Vec<f64>,pub touched_paths:Vec<String>}
pub async fn derive(operation:&GltfChangeNodeMorphWeightsPayload,base:&GltfSnapshot)->Result<GltfChangeNodeMorphWeightsInverse,GltfTopLevelMutationRejection>{validate(operation,base)?;let before=base.document.nodes[operation.node].weights.clone();Ok(GltfChangeNodeMorphWeightsInverse{operation:operation.clone(),before,touched_paths:["document/nodes/*/weights"].into_iter().map(str::to_owned).collect()})}
pub async fn apply(base:&GltfSnapshot,inverse:&GltfChangeNodeMorphWeightsInverse)->Result<GltfSnapshot,GltfTopLevelMutationRejection>{let mut next=base.clone();next.document.nodes[diff.operation.node].weights=inverse.before.clone();Ok(next)}
pub async fn encode(inverse:&GltfChangeNodeMorphWeightsInverse)->Result<Vec<u8>,serde_json::Error>{serde_json::to_vec(inverse)}
