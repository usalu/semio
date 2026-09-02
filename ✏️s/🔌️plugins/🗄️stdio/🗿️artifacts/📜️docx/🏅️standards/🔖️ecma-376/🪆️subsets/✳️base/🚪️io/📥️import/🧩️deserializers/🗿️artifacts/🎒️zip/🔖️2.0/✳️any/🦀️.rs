//! Deserialize stdio.docx from stdio.binary (parse ZIP bytes).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::docx::{DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🎒️ Parse ZIP container bytes into a DocxSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<DocxSnapshot, store::PackError> {
    let mut snap = crate::artifacts::docx::engine::decode_docx(&from.bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_DOCX_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DocxSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
