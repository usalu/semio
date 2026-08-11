//! Serialize stdio.pdf to stdio.deflate.

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;

pub fn register() {}

pub fn serialize(from: &PdfSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let bytes = crate::artifacts::pdf::standards::v1_4::engine::encode_pdf(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot {
        schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(),
        compression_method: 8,
        window_bits: 7,
        compression_level_hint: crate::artifacts::deflate::schema::snapshot::DeflateLevelHint::default(),
        dict_id: None,
        payload: bytes,
    })
}
