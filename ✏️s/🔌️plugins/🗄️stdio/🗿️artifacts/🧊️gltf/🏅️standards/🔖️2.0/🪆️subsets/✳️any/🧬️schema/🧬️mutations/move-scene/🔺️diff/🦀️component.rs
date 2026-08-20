//! 🔺️ move-scene leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::move_scene::mutation::{apply, validate, GltfMoveScenePayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{scenes_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.move-scene.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfMoveSceneOperation { Insert { position: usize, item: GltfScene }, Delete { index: usize, removed: GltfScene }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveSceneDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfMoveScenePayload, pub operation: GltfMoveSceneOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfMoveScenePayload, base: &GltfSnapshot) -> GltfMoveSceneOperation { GltfMoveSceneOperation::Move { index: payload.index, position: payload.position } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfMoveScenePayload) -> Vec<String> { vec![format!("document/scenes/{}", payload.index), format!("document/scenes/{}", payload.position)] }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfMoveSceneDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfMoveSceneDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfMoveSceneOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Scenes, &Change::Insert(*position))?; next.document.scenes.insert(*position, item.clone());  }, GltfMoveSceneOperation::Delete { index, .. } => { scenes_op(&mut next, GltfTopLevelFamily::Scenes, *index, None, None)?;  }, GltfMoveSceneOperation::Move { index, position } => { scenes_op(&mut next, GltfTopLevelFamily::Scenes, *index, Some(*position), None)?;  }, GltfMoveSceneOperation::Reorder { order } => { scenes_op(&mut next, GltfTopLevelFamily::Scenes, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfMoveSceneDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfMoveScenePayload, base: &GltfSnapshot) -> Result<GltfMoveSceneDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfMoveSceneDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfMoveScenePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Scenes, base, &next)) }
