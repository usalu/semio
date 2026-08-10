//! deser ifc via binary
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::ifc::IfcSnapshot;
pub fn register() {}
pub fn deserialize(from: &BinarySnapshot) -> Result<IfcSnapshot, store::PackError> {
    let text = String::from_utf8(from.bytes.clone()).map_err(|e| store::PackError::Schema(e.to_string()))?;
    <IfcSnapshot as store::DocumentDsl>::parse_dsl(&text).map_err(|e| store::PackError::Schema(e.to_string()))
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<IfcSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
