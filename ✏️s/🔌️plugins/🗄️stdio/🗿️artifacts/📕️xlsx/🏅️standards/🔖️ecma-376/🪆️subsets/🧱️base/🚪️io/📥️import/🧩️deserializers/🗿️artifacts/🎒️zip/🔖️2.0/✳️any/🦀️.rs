//! Deserialize stdio.xlsx from stdio.binary (parse ZIP bytes).

use crate::{XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_binary::BinarySnapshot;

//#region Codec
/// Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🎒️ Parse ZIP container bytes into a XlsxSnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<XlsxSnapshot, store::PackError> {
    let mut snap = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx(&from.bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_XLSX_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<XlsxSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
