//! Deserialize stdio.pptx from stdio.binary (parse ZIP bytes).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::pptx::{PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🎒️ Parse ZIP container bytes into a PptxSnapshot.
pub fn deserialize(from: &BinarySnapshot) -> Result<PptxSnapshot, store::PackError> {
    let mut snap = crate::artifacts::pptx::engine::decode_pptx(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_PPTX_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<PptxSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
//#endregion Codec
