//! Deserialize stdio.pdf from stdio.deflate (raw file bytes in deflate snapshot).

use semio_s_artifact_stdio_deflate::DeflateSnapshot;
use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use crate::STDIO_PDF_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &DeflateSnapshot) -> Result<PdfSnapshot, store::PackError> {
    let mut snap = crate::standards::v1_4::subsets::base::io::decode_pdf(&from.payload).map_err(store::PackError::Schema)?;
    snap.schema = STDIO_PDF_DOCUMENT_SCHEMA.into();
    Ok(snap)
}
