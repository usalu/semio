//! Deserialize stdio.glb from stdio.binary (parse ZIP bytes).

use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🎒️ Parse ZIP container bytes into a GlbSnapshot.
pub fn decode_glb_from_json(from: &JsonSnapshot) -> Result<GlbSnapshot, store::PackError> {
    let snap = GlbSnapshot { schema: STDIO_GLB_DOCUMENT_SCHEMA.into(), payload: crate::artifacts::glb::schema::snapshot::GlbPayload { gltf_json: serde_json::to_string(&from.value).unwrap_or_default(), bin: Vec::new() } };
    Ok(snap)
}

pub fn deserialize(from: &JsonSnapshot) -> Result<GlbSnapshot, store::PackError> {
    let mut snap = crate::artifacts::glb::engine::decode_glb(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_GLB_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<GlbSnapshot, store::PackError> {
    deserialize(&<JsonSnapshot as store::DocumentPack>::decode_glb_from_json(from)?)
}
//#endregion Codec
