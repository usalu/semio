//! Deserialize stdio.pdf from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::pdf::{PdfSnapshot, STDIO_PDF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<PdfSnapshot, store::PackError> {
    let mut snap = crate::artifacts::pdf::engine::decode_pdf(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_PDF_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<PdfSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
