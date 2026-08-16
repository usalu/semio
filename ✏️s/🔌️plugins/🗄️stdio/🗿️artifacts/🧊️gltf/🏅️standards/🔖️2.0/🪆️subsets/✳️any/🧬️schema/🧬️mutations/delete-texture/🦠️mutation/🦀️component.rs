//! 🦠️ delete-texture typed structural command with reference repair.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::*;
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::*;
pub const ID: &str = "s.stdio.gltf.mutation.delete-texture.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/textures"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)] #[serde(rename_all = "camelCase")]
pub struct GltfDeleteTexturePayload { pub index: usize }
pub fn validate(payload: &GltfDeleteTexturePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.index >= base.document.textures.len() { return Err(reject("gltf.mutation.index-out-of-range", "document/textures", "index must address an item")); }  Ok(()) }
pub fn apply(payload: &GltfDeleteTexturePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); textures_op(&mut next, GltfTopLevelFamily::Textures, payload.index, None, None)?;  Ok(next) }
