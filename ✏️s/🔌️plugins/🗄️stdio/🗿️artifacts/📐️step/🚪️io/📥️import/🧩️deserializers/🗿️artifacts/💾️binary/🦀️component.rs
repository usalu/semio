//! deser step via binary
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::step::{StepSnapshot, STDIO_STEP_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &BinarySnapshot) -> Result<StepSnapshot, store::PackError> {
    let text = String::from_utf8(from.bytes.clone()).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(StepSnapshot { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), text })
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<StepSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
