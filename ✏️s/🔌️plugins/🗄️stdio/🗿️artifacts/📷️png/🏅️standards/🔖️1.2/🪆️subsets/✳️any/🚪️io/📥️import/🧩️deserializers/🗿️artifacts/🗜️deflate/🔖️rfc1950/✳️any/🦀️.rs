//! Deserialize stdio.png from stdio.deflate (raw file bytes in deflate snapshot).

use crate::artifacts::deflate::DeflateSnapshot;
use crate::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &DeflateSnapshot) -> Result<PngSnapshot, store::PackError> {
    let mut snap = crate::artifacts::png::engine::decode_png(&from.payload).map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_PNG_DOCUMENT_SCHEMA.into();
    Ok(snap)
}
