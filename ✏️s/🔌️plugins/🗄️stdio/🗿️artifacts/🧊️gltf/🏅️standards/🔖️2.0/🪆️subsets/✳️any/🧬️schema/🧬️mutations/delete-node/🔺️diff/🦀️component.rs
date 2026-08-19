//! 🔺️ delete-node leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfNode;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::delete_node::mutation::{apply, validate, GltfDeleteNodePayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{nodes_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.delete-node.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfDeleteNodeOperation { Insert { position: usize, item: GltfNode }, Delete { index: usize, removed: GltfNode }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteNodeDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfDeleteNodePayload, pub operation: GltfDeleteNodeOperation }
async fn operation(payload: &GltfDeleteNodePayload, base: &GltfSnapshot) -> GltfDeleteNodeOperation { GltfDeleteNodeOperation::Delete { index: payload.index, removed: base.document.nodes[payload.index].clone() } }
async fn touched_paths(payload: &GltfDeleteNodePayload) -> Vec<String> { vec![format!("document/nodes/{}", payload.index)] }
pub async fn validate_diff(diff: &GltfDeleteNodeDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
pub async fn apply_diff(diff: &GltfDeleteNodeDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfDeleteNodeOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Nodes, &Change::Insert(*position))?; next.document.nodes.insert(*position, item.clone());  }, GltfDeleteNodeOperation::Delete { index, .. } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, *index, None, None)?;  }, GltfDeleteNodeOperation::Move { index, position } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, *index, Some(*position), None)?;  }, GltfDeleteNodeOperation::Reorder { order } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, order[0], None, Some(order))?;  } } Ok(next) }
pub async fn encode(diff: &GltfDeleteNodeDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
pub async fn derive(payload: &GltfDeleteNodePayload, base: &GltfSnapshot) -> Result<GltfDeleteNodeDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfDeleteNodeDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
pub async fn derive_transitional_gltf_diff(payload: &GltfDeleteNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Nodes, base, &next)) }
