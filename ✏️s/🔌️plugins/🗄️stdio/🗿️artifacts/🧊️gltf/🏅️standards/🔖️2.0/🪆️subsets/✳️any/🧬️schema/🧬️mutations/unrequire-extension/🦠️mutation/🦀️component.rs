//! 🦠️ unrequire-extension executable typed payload and validation.
use serde::{Deserialize, Serialize};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::schema::mutations::top_level_private::{reject, GltfTopLevelMutationRejection};
pub const ID: &str = "s.stdio.gltf.mutation.unrequire-extension.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfUnrequireExtensionPayload { pub extension: String }
pub fn validate(payload: &GltfUnrequireExtensionPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> { if !base.document.extensions_required.contains(&payload.extension) { return Err(reject("gltf.mutation.extension-absent", "document/extensionsRequired", "extension is not declared")); }  Ok(()) }
pub fn apply(payload: &GltfUnrequireExtensionPayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> { validate(payload, base)?; let mut next = base.clone(); next.document.extensions_required.retain(|value| value != &payload.extension); Ok(next) }
