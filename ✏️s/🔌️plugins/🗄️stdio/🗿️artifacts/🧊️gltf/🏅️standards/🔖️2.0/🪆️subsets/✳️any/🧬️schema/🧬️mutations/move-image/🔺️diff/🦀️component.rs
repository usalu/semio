//! 🔺️ move-image leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfImage;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::move_image::mutation::{apply, validate, GltfMoveImagePayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{images_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.move-image.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfMoveImageOperation { Insert { position: usize, item: GltfImage }, Delete { index: usize, removed: GltfImage }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfMoveImageDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfMoveImagePayload, pub operation: GltfMoveImageOperation }
fn operation(payload: &GltfMoveImagePayload, base: &GltfSnapshot) -> GltfMoveImageOperation { GltfMoveImageOperation::Move { index: payload.index, position: payload.position } }
fn touched_paths(payload: &GltfMoveImagePayload) -> Vec<String> { vec![format!("document/images/{}", payload.index), format!("document/images/{}", payload.position)] }
pub fn validate_diff(diff: &GltfMoveImageDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
pub fn apply_diff(diff: &GltfMoveImageDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfMoveImageOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Images, &Change::Insert(*position))?; next.document.images.insert(*position, item.clone());  }, GltfMoveImageOperation::Delete { index, .. } => { images_op(&mut next, GltfTopLevelFamily::Images, *index, None, None)?;  }, GltfMoveImageOperation::Move { index, position } => { images_op(&mut next, GltfTopLevelFamily::Images, *index, Some(*position), None)?;  }, GltfMoveImageOperation::Reorder { order } => { images_op(&mut next, GltfTopLevelFamily::Images, order[0], None, Some(order))?;  } } Ok(next) }
pub fn encode(diff: &GltfMoveImageDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
pub fn derive(payload: &GltfMoveImagePayload, base: &GltfSnapshot) -> Result<GltfMoveImageDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfMoveImageDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
pub fn derive_transitional_gltf_diff(payload: &GltfMoveImagePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Images, base, &next)) }
