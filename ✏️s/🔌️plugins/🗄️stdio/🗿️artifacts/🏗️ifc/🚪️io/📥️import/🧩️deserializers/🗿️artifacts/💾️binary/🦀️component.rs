//! deser ifc via binary
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::ifc::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &BinarySnapshot) -> Result<IfcSnapshot, store::PackError> {
    let text = String::from_utf8(from.bytes.clone()).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(IfcSnapshot { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), text })
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<IfcSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
