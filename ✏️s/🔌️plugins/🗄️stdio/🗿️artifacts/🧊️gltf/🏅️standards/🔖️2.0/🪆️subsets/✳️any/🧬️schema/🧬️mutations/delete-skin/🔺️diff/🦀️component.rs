//! 🔺️ delete-skin leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfSkin;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::delete_skin::mutation::{apply, validate, GltfDeleteSkinPayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{skins_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.delete-skin.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfDeleteSkinOperation { Insert { position: usize, item: GltfSkin }, Delete { index: usize, removed: GltfSkin }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteSkinDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfDeleteSkinPayload, pub operation: GltfDeleteSkinOperation }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn operation(payload: &GltfDeleteSkinPayload, base: &GltfSnapshot) -> GltfDeleteSkinOperation { GltfDeleteSkinOperation::Delete { index: payload.index, removed: base.document.skins[payload.index].clone() } }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn touched_paths(payload: &GltfDeleteSkinPayload) -> Vec<String> { vec![format!("document/skins/{}", payload.index)] }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_diff(diff: &GltfDeleteSkinDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_diff(diff: &GltfDeleteSkinDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfDeleteSkinOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Skins, &Change::Insert(*position))?; next.document.skins.insert(*position, item.clone());  }, GltfDeleteSkinOperation::Delete { index, .. } => { skins_op(&mut next, GltfTopLevelFamily::Skins, *index, None, None)?;  }, GltfDeleteSkinOperation::Move { index, position } => { skins_op(&mut next, GltfTopLevelFamily::Skins, *index, Some(*position), None)?;  }, GltfDeleteSkinOperation::Reorder { order } => { skins_op(&mut next, GltfTopLevelFamily::Skins, order[0], None, Some(order))?;  } } Ok(next) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode(diff: &GltfDeleteSkinDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive(payload: &GltfDeleteSkinPayload, base: &GltfSnapshot) -> Result<GltfDeleteSkinDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfDeleteSkinDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn derive_transitional_gltf_diff(payload: &GltfDeleteSkinPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Skins, base, &next)) }
