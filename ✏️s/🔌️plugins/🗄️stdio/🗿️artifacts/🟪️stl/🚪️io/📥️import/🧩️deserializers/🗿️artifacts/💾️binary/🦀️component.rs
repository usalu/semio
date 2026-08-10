//! 📥️ Deserialize `stdio.stl` from stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<StlSnapshot, store::PackError> {
    let (vertices, faces) = crate::artifacts::stl::schema::snapshot::parse_stl_bytes(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices, faces })
}
