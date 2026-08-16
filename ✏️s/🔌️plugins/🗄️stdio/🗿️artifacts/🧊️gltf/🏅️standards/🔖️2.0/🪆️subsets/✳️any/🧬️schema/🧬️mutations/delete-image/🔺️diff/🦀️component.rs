//! 🔺️ delete-image leaf-owned typed sparse operation diff.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::snapshot::GltfImage;
use crate::artifacts::gltf::schema::diff::GltfDiff;
use crate::artifacts::gltf::schema::mutations::delete_image::mutation::{apply, validate, GltfDeleteImagePayload};
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{images_op, family_diff, reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.delete-image.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(tag = "kind", rename_all = "camelCase")]
pub enum GltfDeleteImageOperation { Insert { position: usize, item: GltfImage }, Delete { index: usize, removed: GltfImage }, Move { index: usize, position: usize }, Reorder { order: Vec<usize> } }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteImageDiff { pub id: String, pub version: u32, pub touched_paths: Vec<String>, pub payload: GltfDeleteImagePayload, pub operation: GltfDeleteImageOperation }
fn operation(payload: &GltfDeleteImagePayload, base: &GltfSnapshot) -> GltfDeleteImageOperation { GltfDeleteImageOperation::Delete { index: payload.index, removed: base.document.images[payload.index].clone() } }
fn touched_paths(payload: &GltfDeleteImagePayload) -> Vec<String> { vec![format!("document/images/{}", payload.index)] }
pub fn validate_diff(diff: &GltfDeleteImageDiff, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if diff.id != ID || diff.version != 1 { return Err(reject("gltf.mutation.invalid-diff-envelope", "diff", "descriptor identity does not match")); } if diff.touched_paths != touched_paths(&diff.payload) { return Err(reject("gltf.mutation.invalid-touched-paths", "diff/touchedPaths", "paths must be concrete payload-derived locations")); } validate(&diff.payload, base)?; if diff.operation != operation(&diff.payload, base) { return Err(reject("gltf.mutation.invalid-sparse-operation", "diff/operation", "operation must equal the direct typed delta")); } Ok(()) }
pub fn apply_diff(diff: &GltfDeleteImageDiff, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate_diff(diff, base)?; let mut next = base.clone(); let operation = &diff.operation; match operation { GltfDeleteImageOperation::Insert { position, item } => { repair(&mut next.document, GltfTopLevelFamily::Images, &Change::Insert(*position))?; next.document.images.insert(*position, item.clone());  }, GltfDeleteImageOperation::Delete { index, .. } => { images_op(&mut next, GltfTopLevelFamily::Images, *index, None, None)?;  }, GltfDeleteImageOperation::Move { index, position } => { images_op(&mut next, GltfTopLevelFamily::Images, *index, Some(*position), None)?;  }, GltfDeleteImageOperation::Reorder { order } => { images_op(&mut next, GltfTopLevelFamily::Images, order[0], None, Some(order))?;  } } Ok(next) }
pub fn encode(diff: &GltfDeleteImageDiff) -> Result<Vec<u8>, GltfTopLevelMutationRejection> { serde_json::to_vec(diff).map_err(|error| reject("gltf.mutation.encode-failed", "diff", error.to_string())) }
pub fn derive(payload: &GltfDeleteImagePayload, base: &GltfSnapshot) -> Result<GltfDeleteImageDiff, GltfTopLevelMutationRejection> { validate(payload, base)?; Ok(GltfDeleteImageDiff { id: ID.into(), version: 1, touched_paths: touched_paths(payload), payload: payload.clone(), operation: operation(payload, base) }) }
pub fn derive_transitional_gltf_diff(payload: &GltfDeleteImagePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> { let next = apply(payload, base)?; Ok(family_diff(GltfTopLevelFamily::Images, base, &next)) }
