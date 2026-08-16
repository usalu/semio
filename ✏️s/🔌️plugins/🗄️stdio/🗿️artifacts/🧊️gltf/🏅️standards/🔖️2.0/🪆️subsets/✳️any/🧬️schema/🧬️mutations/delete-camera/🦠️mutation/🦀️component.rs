//! 🦠️ delete-camera typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.delete-camera.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/cameras"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteCameraPayload { pub index: usize }
pub fn validate(payload: &GltfDeleteCameraPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.cameras.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/cameras", "index must address an item")); }  Ok(()) }
pub fn apply(payload: &GltfDeleteCameraPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); cameras_op(&mut next, GltfTopLevelFamily::Cameras, payload.index, None, None)?;  Ok(next) }
