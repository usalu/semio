//! Serialize stdio.png to stdio.deflate.

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::png::PngSnapshot;

pub fn register() {}

pub fn serialize(from: &PngSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let bytes = crate::artifacts::png::engine::encode_png(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), bytes })
}
