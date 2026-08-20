//! 🔺️ move-mesh leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfMesh;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::move_mesh::mutation::{apply, validate, GltfMoveMeshPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{meshes_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.move-mesh.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfMoveMeshOperation { Insert { position: usize, item: GltfMesh }, Delete { index: usize, removed: GltfMesh }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveMeshDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfMoveMeshPayload, pub operation: GltfMoveMeshOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfMoveMeshPayload, base: &GltfSnapshot) -> GltfMoveMeshOperation { GltfMoveMeshOperation::Move { index: payload.index, position: payload.position } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfMoveMeshPayload) -> Vec<String> { vec![format!("document/meshes/{}", payload.index), format!("document/meshes/{}", payload.position)] }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfMoveMeshDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfMoveMeshDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfMoveMeshOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Meshes, &Change::Insert(*position))?; next.document.meshes.insert(*position, item.clone());  }, GltfMoveMeshOperation::Delete { index, .. } => { meshes_op(&mut next, GltfTopLevelFamily::Meshes, *index, None, None)?;  }, GltfMoveMeshOperation::Move { index, position } => { meshes_op(&mut next, GltfTopLevelFamily::Meshes, *index, Some(*position), None)?;  }, GltfMoveMeshOperation::Reorder { order } => { meshes_op(&mut next, GltfTopLevelFamily::Meshes, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfMoveMeshDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfMoveMeshPayload, base: &GltfSnapshot) -> Result<GltfMoveMeshDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfMoveMeshDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfMoveMeshPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Meshes, base, &next)) }
