//! Serialize stdio.glb to stdio.binary (encode ZIP bytes).

use crate::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};
use crate::artifacts::glb::GlbSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// 🎒️ Encode GlbSnapshot as ZIP container bytes.
pub fn serialize(from: &GlbSnapshot) -> Result<JsonSnapshot, store::PackError> {
    let bytes = crate::artifacts::glb::engine::encode_glb(from, true)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(JsonSnapshot {
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        bytes,
    })
}

/// Encode ZIP then wrap as binary pack bytes.
pub fn serialize_bytes(from: &GlbSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::DocumentPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
