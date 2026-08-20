//! 🔺️ reorder-animations leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfAnimation;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::reorder_animations::mutation::{apply, validate, GltfReorderAnimationsPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{animations_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-animations.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfReorderAnimationsOperation { Insert { position: usize, item: GltfAnimation }, Delete { index: usize, removed: GltfAnimation }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderAnimationsDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfReorderAnimationsPayload, pub operation: GltfReorderAnimationsOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfReorderAnimationsPayload, base: &GltfSnapshot) -> GltfReorderAnimationsOperation { GltfReorderAnimationsOperation::Reorder { order: payload.order.clone() } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfReorderAnimationsPayload) -> Vec<String> { payload.order.iter().map(|index| format!("document/animations/{}", index)).chain(std::iter::empty()).collect() }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfReorderAnimationsDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfReorderAnimationsDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfReorderAnimationsOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Animations, &Change::Insert(*position))?; next.document.animations.insert(*position, item.clone());  }, GltfReorderAnimationsOperation::Delete { index, .. } => { animations_op(&mut next, GltfTopLevelFamily::Animations, *index, None, None)?;  }, GltfReorderAnimationsOperation::Move { index, position } => { animations_op(&mut next, GltfTopLevelFamily::Animations, *index, Some(*position), None)?;  }, GltfReorderAnimationsOperation::Reorder { order } => { animations_op(&mut next, GltfTopLevelFamily::Animations, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfReorderAnimationsDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfReorderAnimationsPayload, base: &GltfSnapshot) -> Result<GltfReorderAnimationsDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfReorderAnimationsDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfReorderAnimationsPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Animations, base, &next)) }
