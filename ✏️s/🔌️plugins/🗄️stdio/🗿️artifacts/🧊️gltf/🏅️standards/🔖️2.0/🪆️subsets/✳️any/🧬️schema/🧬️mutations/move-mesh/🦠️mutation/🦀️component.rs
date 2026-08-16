//! 🦠️ move-mesh typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-mesh.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/meshes"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveMeshPayload { pub index: usize, pub position: usize }
pub fn validate(payload: &GltfMoveMeshPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.meshes.len() || payload.position >= base.document.meshes.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/meshes", "indices must address items")); } if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/meshes", "destination equals source")); }  Ok(()) }
pub fn apply(payload: &GltfMoveMeshPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); meshes_op(&mut next, GltfTopLevelFamily::Meshes, payload.index, Some(payload.position), None)?;  Ok(next) }
