//! 🦠️ change-document-extension-data executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.change-document-extension-data.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensions"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeDocumentExtensionDataPayload { pub data: Option<GltfJson> }
pub fn validate(payload: &GltfChangeDocumentExtensionDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if payload.data == base.document.extensions { return Err(reject("gltf.mutation.no-observable-change", "document/extensions", "value already has this value")); } Ok(()) }
pub fn apply(payload: &GltfChangeDocumentExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions = payload.data.clone(); Ok(next) }
