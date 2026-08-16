//! 🦠️ change-asset-extra-data executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.change-asset-extra-data.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/asset/extras"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeAssetExtraDataPayload { pub data: Option<GltfJson> }
pub fn validate(payload: &GltfChangeAssetExtraDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.data == base.document.asset.extras { return Err(reject("gltf.mutation.no-observable-change", "document/asset/extras", "value already has this value")); } Ok(()) }
pub fn apply(payload: &GltfChangeAssetExtraDataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.asset.extras = payload.data.clone(); Ok(next) }
