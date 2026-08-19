//! 🦠️ bind-default-scene executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.bind-default-scene.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/scene"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfBindDefaultScenePayload { pub scene: usize }
pub async fn validate(payload: &GltfBindDefaultScenePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.scene >= base.document.scenes.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "scene must exist")); } if Some(payload.scene) == base.document.scene { return Err(reject("gltf.mutation.no-observable-change", "document/scene", "scene is already default")); } Ok(()) }
pub async fn apply(payload: &GltfBindDefaultScenePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.scene = Some(payload.scene); Ok(next) }
