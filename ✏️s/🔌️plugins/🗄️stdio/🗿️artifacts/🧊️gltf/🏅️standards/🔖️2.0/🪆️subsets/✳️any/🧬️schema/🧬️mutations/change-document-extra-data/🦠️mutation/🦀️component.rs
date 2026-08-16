//! 🦠️ change-document-extra-data executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.change-document-extra-data.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extras"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeDocumentExtraDataPayload { pub data: Option<GltfJson> }
pub fn validate(payload: &GltfChangeDocumentExtraDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.data == base.document.extras { return Err(reject("gltf.mutation.no-observable-change", "document/extras", "value already has this value")); } Ok(()) }
pub fn apply(payload: &GltfChangeDocumentExtraDataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extras = payload.data.clone(); Ok(next) }
