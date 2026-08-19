//! 🦠️ create-skin typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.create-skin.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/skins"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfCreateSkinPayload { pub position: usize }
pub async fn validate(payload: &GltfCreateSkinPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.position > base.document.skins.len() { return Err(reject("gltf.mutation.insert-out-of-range", "document/skins", "position must be within the collection")); }   Ok(()) }
pub async fn apply(payload: &GltfCreateSkinPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); repair(&mut next.document, GltfTopLevelFamily::Skins, &Change::Insert(payload.position))?; next.document.skins.insert(payload.position, GltfSkin::default()); Ok(next) }
