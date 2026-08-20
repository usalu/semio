//! 🔺️ reorder-nodes leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfNode;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::reorder_nodes::mutation::{apply, validate, GltfReorderNodesPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{nodes_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-nodes.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfReorderNodesOperation { Insert { position: usize, item: GltfNode }, Delete { index: usize, removed: GltfNode }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderNodesDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfReorderNodesPayload, pub operation: GltfReorderNodesOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfReorderNodesPayload, base: &GltfSnapshot) -> GltfReorderNodesOperation { GltfReorderNodesOperation::Reorder { order: payload.order.clone() } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfReorderNodesPayload) -> Vec<String> { payload.order.iter().map(|index| format!("document/nodes/{}", index)).chain(std::iter::empty()).collect() }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfReorderNodesDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfReorderNodesDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfReorderNodesOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Nodes, &Change::Insert(*position))?; next.document.nodes.insert(*position, item.clone());  }, GltfReorderNodesOperation::Delete { index, .. } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, *index, None, None)?;  }, GltfReorderNodesOperation::Move { index, position } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, *index, Some(*position), None)?;  }, GltfReorderNodesOperation::Reorder { order } => { nodes_op(&mut next, GltfTopLevelFamily::Nodes, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfReorderNodesDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfReorderNodesPayload, base: &GltfSnapshot) -> Result<GltfReorderNodesDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfReorderNodesDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfReorderNodesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Nodes, base, &next)) }
