//! Serialize stdio.jpg to stdio.binary.

use crate::JpgSnapshot;
use semio_s_artifact_stdio_binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &JpgSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::standards::v_jfif_1_01::subsets::document::io::encode_jpg(from, &crate::standards::v_jfif_1_01::subsets::document::io::JpgEncodeOptions::from_frame(from.frame.as_ref())).map_err(|e| store::PackError::from(semio_framework_value::ValueError::from(e)))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
