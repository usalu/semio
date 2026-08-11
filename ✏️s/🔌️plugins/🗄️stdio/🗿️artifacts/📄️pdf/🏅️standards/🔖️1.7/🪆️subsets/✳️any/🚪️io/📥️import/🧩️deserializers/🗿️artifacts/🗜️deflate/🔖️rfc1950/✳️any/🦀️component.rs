//! Deserialize stdio.pdf (1.7) from stdio.deflate (raw file bytes in deflate snapshot).

use crate::artifacts::deflate::DeflateSnapshot;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &DeflateSnapshot) -> Result<PdfSnapshot, store::PackError> {
    let mut snap = crate::artifacts::pdf::standards::v1_7::engine::decode_pdf(&from.payload)
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
    snap.schema = STDIO_PDF17_DOCUMENT_SCHEMA.into();
    Ok(snap)
}
