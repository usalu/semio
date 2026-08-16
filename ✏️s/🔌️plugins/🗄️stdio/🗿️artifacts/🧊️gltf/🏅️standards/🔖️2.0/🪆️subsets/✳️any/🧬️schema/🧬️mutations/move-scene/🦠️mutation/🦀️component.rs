//! 🦠️ move-scene typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.move-scene.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/scenes"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveScenePayload { pub index: usize, pub position: usize }
pub fn validate(payload: &GltfMoveScenePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.scenes.len() || payload.position >= base.document.scenes.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "indices must address items")); } if payload.index == payload.position { return Err(reject("gltf.mutation.no-observable-change", "document/scenes", "destination equals source")); }  Ok(()) }
pub fn apply(payload: &GltfMoveScenePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); scenes_op(&mut next, GltfTopLevelFamily::Scenes, payload.index, Some(payload.position), None)?;  Ok(next) }
