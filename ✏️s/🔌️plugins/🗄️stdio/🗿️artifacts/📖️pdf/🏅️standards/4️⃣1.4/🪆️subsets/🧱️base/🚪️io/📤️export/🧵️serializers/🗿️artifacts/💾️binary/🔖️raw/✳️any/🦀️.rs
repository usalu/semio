//! Serialize stdio.pdf to stdio.binary.

use crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot;
use semio_s_artifact_stdio_binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &PdfSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::standards::v1_4::subsets::base::io::encode_pdf(from).map_err(store::PackError::Schema)?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
