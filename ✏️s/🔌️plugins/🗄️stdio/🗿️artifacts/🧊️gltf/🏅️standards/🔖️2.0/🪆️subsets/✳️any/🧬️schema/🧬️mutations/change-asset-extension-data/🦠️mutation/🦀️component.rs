//! 🦠️ change-asset-extension-data executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-extension-data.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/extensions"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeAssetExtensionDataPayload { pub data: Option<GltfJson> }
pub async fn validate(payload: &GltfChangeAssetExtensionDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.data == base.document.asset.extensions { return Err(reject("gltf.mutation.no-observable-change", "document/asset/extensions", "value already has this value")); } Ok(()) }
pub async fn apply(payload: &GltfChangeAssetExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.asset.extensions = payload.data.clone(); Ok(next) }
