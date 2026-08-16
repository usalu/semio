//! 🔺️ create-node leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfNode;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::create_node::mutation::{apply, validate, GltfCreateNodePayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{nodes_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.create-node.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfCreateNodeOperation { Insert { position: usize, item: GltfNode }, Delete { index: usize, removed: GltfNode }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateNodeDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfCreateNodePayload, pub operation: GltfCreateNodeOperation }
fn operation(payload: &GltfCreateNodePayload, base: &GltfSnapshot) -> GltfCreateNodeOperation { GltfCreateNodeOperation::Insert { position: payload.position, item: GltfNode::default() } }
fn touched_paths(payload: &GltfCreateNodePayload) -> Vec<String> { vec![format!("document/nodes/{}", payload.position)] }
pub fn validate_diff(diff: &GltfCreateNodeDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
pub fn apply_diff(diff: &GltfCreateNodeDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfCreateNodeOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Nodes, &Change::Insert(*position))?; next.document.nodes.insert(*position, item.clone());  }, GltfCreateNodeOperation::Delete { index, .. } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, *index, None, None)?;  }, GltfCreateNodeOperation::Move { index, position } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, *index, Some(*position), None)?;  }, GltfCreateNodeOperation::Reorder { order } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, order[0], None, Some(order))?;  } } Ok(next) }
pub fn encode(diff: &GltfCreateNodeDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
pub fn derive(payload: &GltfCreateNodePayload, base: &GltfSnapshot) -> Result<GltfCreateNodeDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfCreateNodeDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
pub fn derive_transitional_gltf_diff(payload: &GltfCreateNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Nodes, base, &next)) }
