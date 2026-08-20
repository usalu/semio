//! Serialize stdio.png to stdio.deflate.

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::png::PngSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &PngSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let bytes = crate::artifacts::png::engine::encode_png(from).map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: crate::artifacts::deflate::schema::snapshot::DeflateLevelHint::default(), dict_id: None, payload: bytes })
}
