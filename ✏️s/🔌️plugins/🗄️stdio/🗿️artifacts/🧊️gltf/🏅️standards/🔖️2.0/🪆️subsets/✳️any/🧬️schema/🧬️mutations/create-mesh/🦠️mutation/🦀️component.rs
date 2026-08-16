//! 🦠️ create-mesh typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-mesh.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/meshes"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateMeshPayload { pub position: usize }
pub fn validate(payload: &GltfCreateMeshPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.meshes.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/meshes", "position must be within the collection")); }   Ok(()) }
pub fn apply(payload: &GltfCreateMeshPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Meshes, &Change::Insert(payload.position))?; next.document.meshes.insert(payload.position, GltfMesh::default()); Ok(next) }
