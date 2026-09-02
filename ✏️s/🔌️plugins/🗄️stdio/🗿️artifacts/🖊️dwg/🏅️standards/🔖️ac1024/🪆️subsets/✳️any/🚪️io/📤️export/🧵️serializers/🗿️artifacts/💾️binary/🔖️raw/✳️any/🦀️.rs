//! 📤️ Serialize `stdio.dwg` to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::dwg::DwgSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &DwgSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::dwg::schema::snapshot::encode_dwg(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
