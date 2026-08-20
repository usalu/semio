//! 🔺️ delete-buffer leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfBuffer;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::delete_buffer::mutation::{apply, validate, GltfDeleteBufferPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{buffers_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.delete-buffer.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfDeleteBufferOperation { Insert { position: usize, item: GltfBuffer, bytes: Vec<u8> }, Delete { index: usize, removed: GltfBuffer, bytes: Vec<u8> }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteBufferDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfDeleteBufferPayload, pub operation: GltfDeleteBufferOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfDeleteBufferPayload, base: &GltfSnapshot) -> GltfDeleteBufferOperation { GltfDeleteBufferOperation::Delete { index: payload.index, removed: base.document.buffers[payload.index].clone(), bytes: base.buffers[payload.index].clone() } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfDeleteBufferPayload) -> Vec<String> { vec![format!("document/buffers/{}", payload.index), format!("buffers/{}", payload.index)] }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfDeleteBufferDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfDeleteBufferDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfDeleteBufferOperation::Insert { position, item, bytes } => { repair(&mut next.document, GltfTopLevelFamily::Buffers, &Change::Insert(*position))?; next.document.buffers.insert(*position, item.clone()); next.buffers.insert(*position, bytes.clone()); }, GltfDeleteBufferOperation::Delete { index, .. } => { buffers_op(&mut next, GltfTopLevelFamily::Buffers, *index, None, None)?; next.buffers.remove(*index); }, GltfDeleteBufferOperation::Move { index, position } => { buffers_op(&mut next, GltfTopLevelFamily::Buffers, *index, Some(*position), None)?; let bytes = next.buffers.remove(*index); next.buffers.insert(*position, bytes); }, GltfDeleteBufferOperation::Reorder { order } => { buffers_op(&mut next, GltfTopLevelFamily::Buffers, order[0], None, Some(order))?; next.buffers = order.iter().map(|index| next.buffers[*index].clone()).collect(); } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfDeleteBufferDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfDeleteBufferPayload, base: &GltfSnapshot) -> Result<GltfDeleteBufferDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfDeleteBufferDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfDeleteBufferPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Buffers, base, &next)) }
