//! 📤️ Serialize `stdio.dwg` to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;

pub fn register() {}

pub fn serialize(from: &DwgSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::encode_dwg(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
