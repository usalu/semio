//! Deserialize stdio.pdf from stdio.deflate (raw file bytes in deflate snapshot).

use crate::artifacts::deflate::DeflateSnapshot;
use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;

pub fn register() {}

pub fn deserialize(from: &DeflateSnapshot) -> Result<PdfSnapshot, store::PackError> {
    let mut snap = crate::artifacts::pdf::standards::v1_4::engine::decode_pdf(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_PDF_DOCUMENT_SCHEMA.into();
    Ok(snap)
}
