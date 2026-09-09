//! Deserialize stdio.pdf from stdio.binary.

use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use crate::STDIO_PDF_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_binary::BinarySnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<PdfSnapshot, store::PackError> {
    let mut snap = crate::standards::v1_4::subsets::base::io::decode_pdf(&from.bytes).map_err(store::PackError::Schema)?;
    snap.schema = STDIO_PDF_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<PdfSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
