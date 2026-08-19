//! 🦠️ delete-skin typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.delete-skin.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/skins"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteSkinPayload { pub index: usize }
pub async fn validate(payload: &GltfDeleteSkinPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.skins.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/skins", "index must address an item")); }  Ok(()) }
pub async fn apply(payload: &GltfDeleteSkinPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); skins_op(&mut next, GltfTopLevelFamily::Skins, payload.index, None, None)?;  Ok(next) }
