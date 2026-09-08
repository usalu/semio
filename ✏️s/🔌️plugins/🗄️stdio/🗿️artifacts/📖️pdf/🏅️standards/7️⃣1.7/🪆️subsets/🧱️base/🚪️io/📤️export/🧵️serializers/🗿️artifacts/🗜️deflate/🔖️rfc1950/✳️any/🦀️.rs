//! Serialize stdio.pdf (1.7) to stdio.deflate.

use semio_s_artifact_stdio_deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &PdfSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let bytes = crate::standards::v1_7::subsets::base::io::encode_pdf(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: semio_s_artifact_stdio_deflate::schema::snapshot::DeflateLevelHint::default(), dict_id: None, payload: bytes })
}
