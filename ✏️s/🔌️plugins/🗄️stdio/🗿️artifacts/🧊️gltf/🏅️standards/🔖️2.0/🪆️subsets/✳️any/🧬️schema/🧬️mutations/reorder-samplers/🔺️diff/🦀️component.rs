//! 🔺️ reorder-samplers leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfSampler;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::reorder_samplers::mutation::{apply, validate, GltfReorderSamplersPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{samplers_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.reorder-samplers.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfReorderSamplersOperation { Insert { position: usize, item: GltfSampler }, Delete { index: usize, removed: GltfSampler }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfReorderSamplersDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfReorderSamplersPayload, pub operation: GltfReorderSamplersOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfReorderSamplersPayload, base: &GltfSnapshot) -> GltfReorderSamplersOperation { GltfReorderSamplersOperation::Reorder { order: payload.order.clone() } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfReorderSamplersPayload) -> Vec<String> { payload.order.iter().map(|index| format!("document/samplers/{}", index)).chain(std::iter::empty()).collect() }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfReorderSamplersDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfReorderSamplersDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfReorderSamplersOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Samplers, &Change::Insert(*position))?; next.document.samplers.insert(*position, item.clone());  }, GltfReorderSamplersOperation::Delete { index, .. } => { samplers_op(&mut next, GltfTopLevelFamily::Samplers, *index, None, None)?;  }, GltfReorderSamplersOperation::Move { index, position } => { samplers_op(&mut next, GltfTopLevelFamily::Samplers, *index, Some(*position), None)?;  }, GltfReorderSamplersOperation::Reorder { order } => { samplers_op(&mut next, GltfTopLevelFamily::Samplers, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfReorderSamplersDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfReorderSamplersPayload, base: &GltfSnapshot) -> Result<GltfReorderSamplersDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfReorderSamplersDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfReorderSamplersPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Samplers, base, &next)) }
