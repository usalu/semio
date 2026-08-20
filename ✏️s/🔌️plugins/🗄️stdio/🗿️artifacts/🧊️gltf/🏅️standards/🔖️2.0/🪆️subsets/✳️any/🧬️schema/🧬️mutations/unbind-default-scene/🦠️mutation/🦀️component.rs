//! 🦠️ unbind-default-scene executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.unbind-default-scene.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/scene"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfUnbindDefaultScenePayload {  }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfUnbindDefaultScenePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if base.document.scene.is_none() { return Err(reject("gltf.mutation.relation-absent", "document/scene", "no default scene is bound")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(payload: &GltfUnbindDefaultScenePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.scene = None; Ok(next) }
