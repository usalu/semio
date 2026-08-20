//! Deserialize stdio.pdf (1.7) from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<PdfSnapshot, store::PackError> {
    let mut snap = crate::artifacts::pdf::standards::v1_7::subsets::any::io::decode_pdf(&from.bytes).await.map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_PDF17_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<PdfSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
