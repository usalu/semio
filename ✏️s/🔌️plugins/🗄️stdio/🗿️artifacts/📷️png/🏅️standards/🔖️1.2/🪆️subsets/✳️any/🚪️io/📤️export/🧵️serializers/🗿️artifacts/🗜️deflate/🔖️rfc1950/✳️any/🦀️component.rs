//! Serialize stdio.png to stdio.deflate.

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::png::PngSnapshot;

pub async fn register() {}

pub async fn serialize(from: &PngSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let bytes = crate::artifacts::png::engine::encode_png(from).map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: crate::artifacts::deflate::schema::snapshot::DeflateLevelHint::default(), dict_id: None, payload: bytes })
}
