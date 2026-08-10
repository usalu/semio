//! 📥️ Deserialize `stdio.las` from stdio.binary.
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &BinarySnapshot) -> Result<LasSnapshot, store::PackError> {
    let vertices = crate::artifacts::las::schema::snapshot::las_vertices_from_bytes(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), vertices })
}
