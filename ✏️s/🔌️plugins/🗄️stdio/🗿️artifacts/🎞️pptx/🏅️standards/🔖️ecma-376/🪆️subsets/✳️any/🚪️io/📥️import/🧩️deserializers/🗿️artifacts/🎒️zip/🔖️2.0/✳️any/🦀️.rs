//! Deserialize stdio.pptx from stdio.binary (parse ZIP bytes).

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::pptx::{PptxSnapshot, STDIO_PPTX_DOCUMENT_SCHEMA};

//#region Codec
/// Register deserializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🎒️ Parses native archive bytes carried by BinarySnapshot into a presentation.
///
/// This taxonomy leaf materializes native ZIP bytes only while decoding them into the logical
/// PresentationML snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<PptxSnapshot, store::PackError> {
    let mut snap = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_pptx(&from.bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_PPTX_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

/// Decode a Binary pack then parse ZIP.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<PptxSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
//#endregion Codec
