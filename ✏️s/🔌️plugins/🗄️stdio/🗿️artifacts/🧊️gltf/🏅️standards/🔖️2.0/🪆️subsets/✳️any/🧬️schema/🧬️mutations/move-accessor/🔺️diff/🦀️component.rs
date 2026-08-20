//! 🔺️ move-accessor leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfAccessor;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::move_accessor::mutation::{apply, validate, GltfMoveAccessorPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{accessors_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.move-accessor.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfMoveAccessorOperation { Insert { position: usize, item: GltfAccessor }, Delete { index: usize, removed: GltfAccessor }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveAccessorDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfMoveAccessorPayload, pub operation: GltfMoveAccessorOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfMoveAccessorPayload, base: &GltfSnapshot) -> GltfMoveAccessorOperation { GltfMoveAccessorOperation::Move { index: payload.index, position: payload.position } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfMoveAccessorPayload) -> Vec<String> { vec![format!("document/accessors/{}", payload.index), format!("document/accessors/{}", payload.position)] }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfMoveAccessorDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfMoveAccessorDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfMoveAccessorOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Accessors, &Change::Insert(*position))?; next.document.accessors.insert(*position, item.clone());  }, GltfMoveAccessorOperation::Delete { index, .. } => { accessors_op(&mut next, GltfTopLevelFamily::Accessors, *index, None, None)?;  }, GltfMoveAccessorOperation::Move { index, position } => { accessors_op(&mut next, GltfTopLevelFamily::Accessors, *index, Some(*position), None)?;  }, GltfMoveAccessorOperation::Reorder { order } => { accessors_op(&mut next, GltfTopLevelFamily::Accessors, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfMoveAccessorDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfMoveAccessorPayload, base: &GltfSnapshot) -> Result<GltfMoveAccessorDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfMoveAccessorDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfMoveAccessorPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Accessors, base, &next)) }
