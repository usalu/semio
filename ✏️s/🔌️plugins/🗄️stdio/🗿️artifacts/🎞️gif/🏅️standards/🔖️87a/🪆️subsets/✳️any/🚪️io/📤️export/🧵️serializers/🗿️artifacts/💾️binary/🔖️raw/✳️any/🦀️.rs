//! Serialize stdio.gif to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &GifSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::gif::standards::v87a::engine::encode_gif(from).map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
