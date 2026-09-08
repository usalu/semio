//! Serialize stdio.gif to stdio.binary.

use semio_s_artifact_stdio_binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::standards::v87a::subsets::any::schema::snapshot::GifSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &GifSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::standards::v87a::engine::encode_gif(from).map_err(store::PackError::Schema)?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
