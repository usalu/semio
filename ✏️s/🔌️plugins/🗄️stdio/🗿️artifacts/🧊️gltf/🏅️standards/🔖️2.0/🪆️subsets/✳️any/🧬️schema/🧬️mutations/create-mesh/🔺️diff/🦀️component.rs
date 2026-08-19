//! 🔺️ create-mesh leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfMesh;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::create_mesh::mutation::{apply, validate, GltfCreateMeshPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{meshes_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.create-mesh.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfCreateMeshOperation { Insert { position: usize, item: GltfMesh }, Delete { index: usize, removed: GltfMesh }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateMeshDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfCreateMeshPayload, pub operation: GltfCreateMeshOperation }
async fn operation(payload: &GltfCreateMeshPayload, base: &GltfSnapshot) -> GltfCreateMeshOperation { GltfCreateMeshOperation::Insert { position: payload.position, item: GltfMesh::default() } }
async fn touched_paths(payload: &GltfCreateMeshPayload) -> Vec<String> { vec![format!("document/meshes/{}", payload.position)] }
pub async fn validate_diff(diff: &GltfCreateMeshDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
pub async fn apply_diff(diff: &GltfCreateMeshDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfCreateMeshOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Meshes, &Change::Insert(*position))?; next.document.meshes.insert(*position, item.clone());  }, GltfCreateMeshOperation::Delete { index, .. } => { meshes_op(&mut next, GltfTopLevelFamily::Meshes, *index, None, None)?;  }, GltfCreateMeshOperation::Move { index, position } => { meshes_op(&mut next, GltfTopLevelFamily::Meshes, *index, Some(*position), None)?;  }, GltfCreateMeshOperation::Reorder { order } => { meshes_op(&mut next, GltfTopLevelFamily::Meshes, order[0], None, Some(order))?;  } } Ok(next) }
pub async fn encode(diff: &GltfCreateMeshDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
pub async fn derive(payload: &GltfCreateMeshPayload, base: &GltfSnapshot) -> Result<GltfCreateMeshDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfCreateMeshDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
pub async fn derive_transitional_gltf_diff(payload: &GltfCreateMeshPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Meshes, base, &next)) }
