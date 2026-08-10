//! deser binary
use crate::artifacts::binary::BinarySnapshot;
pub fn register() {}
pub fn deserialize(bytes: &[u8]) -> Result<BinarySnapshot, store::PackError> {
    <BinarySnapshot as store::DocumentPack>::decode_pack(bytes)
}
