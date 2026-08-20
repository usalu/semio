//! 🔺️ move-buffer-view leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfBufferView;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::move_buffer_view::mutation::{apply, validate, GltfMoveBufferViewPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{buffer_views_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.move-buffer-view.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfMoveBufferViewOperation { Insert { position: usize, item: GltfBufferView }, Delete { index: usize, removed: GltfBufferView }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveBufferViewDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfMoveBufferViewPayload, pub operation: GltfMoveBufferViewOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfMoveBufferViewPayload, base: &GltfSnapshot) -> GltfMoveBufferViewOperation { GltfMoveBufferViewOperation::Move { index: payload.index, position: payload.position } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfMoveBufferViewPayload) -> Vec<String> { vec![format!("document/bufferViews/{}", payload.index), format!("document/bufferViews/{}", payload.position)] }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfMoveBufferViewDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfMoveBufferViewDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfMoveBufferViewOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::BufferViews, &Change::Insert(*position))?; next.document.buffer_views.insert(*position, item.clone());  }, GltfMoveBufferViewOperation::Delete { index, .. } => { buffer_views_op(&mut next, GltfTopLevelFamily::BufferViews, *index, None, None)?;  }, GltfMoveBufferViewOperation::Move { index, position } => { buffer_views_op(&mut next, GltfTopLevelFamily::BufferViews, *index, Some(*position), None)?;  }, GltfMoveBufferViewOperation::Reorder { order } => { buffer_views_op(&mut next, GltfTopLevelFamily::BufferViews, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfMoveBufferViewDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfMoveBufferViewPayload, base: &GltfSnapshot) -> Result<GltfMoveBufferViewDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfMoveBufferViewDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfMoveBufferViewPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::BufferViews, base, &next)) }
