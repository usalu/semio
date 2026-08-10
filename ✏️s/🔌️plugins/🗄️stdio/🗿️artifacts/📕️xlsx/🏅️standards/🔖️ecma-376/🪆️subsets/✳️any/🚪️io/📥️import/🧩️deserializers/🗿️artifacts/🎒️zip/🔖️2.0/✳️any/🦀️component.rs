//! Deserialize stdio.xlsx from stdio.binary (parse ZIP bytes).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::xlsx::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
pub fn register() {}

/// 🎒️ Parse ZIP container bytes into a XlsxSnapshot.
pub fn deserialize(from: &BinarySnapshot) -> Result<XlsxSnapshot, store::PackError> {
    let mut snap = crate::artifacts::xlsx::engine::decode_xlsx(&from.bytes)
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_XLSX_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<XlsxSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
