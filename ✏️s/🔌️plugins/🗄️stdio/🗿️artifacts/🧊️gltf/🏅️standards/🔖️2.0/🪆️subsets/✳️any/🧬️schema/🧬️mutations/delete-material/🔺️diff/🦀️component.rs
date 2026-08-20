//! 🔺️ delete-material leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfMaterial;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::delete_material::mutation::{apply, validate, GltfDeleteMaterialPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{materials_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.delete-material.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfDeleteMaterialOperation { Insert { position: usize, item: GltfMaterial }, Delete { index: usize, removed: GltfMaterial }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteMaterialDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfDeleteMaterialPayload, pub operation: GltfDeleteMaterialOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfDeleteMaterialPayload, base: &GltfSnapshot) -> GltfDeleteMaterialOperation { GltfDeleteMaterialOperation::Delete { index: payload.index, removed: base.document.materials[payload.index].clone() } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfDeleteMaterialPayload) -> Vec<String> { vec![format!("document/materials/{}", payload.index)] }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfDeleteMaterialDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfDeleteMaterialDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfDeleteMaterialOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Materials, &Change::Insert(*position))?; next.document.materials.insert(*position, item.clone());  }, GltfDeleteMaterialOperation::Delete { index, .. } => { materials_op(&mut next, GltfTopLevelFamily::Materials, *index, None, None)?;  }, GltfDeleteMaterialOperation::Move { index, position } => { materials_op(&mut next, GltfTopLevelFamily::Materials, *index, Some(*position), None)?;  }, GltfDeleteMaterialOperation::Reorder { order } => { materials_op(&mut next, GltfTopLevelFamily::Materials, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfDeleteMaterialDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfDeleteMaterialPayload, base: &GltfSnapshot) -> Result<GltfDeleteMaterialDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfDeleteMaterialDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfDeleteMaterialPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Materials, base, &next)) }
