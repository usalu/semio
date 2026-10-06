//! 📤️ Serialize `stdio.dwg` to stdio.binary.

use crate::DwgSnapshot;
use semio_s_artifact_stdio_binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &DwgSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::standards::v_ac1024::subsets::any::io::binary::snapshot::encode_dwg(from).map_err(|e| store::PackError::from(semio_framework_value::ValueError::from(e)))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
