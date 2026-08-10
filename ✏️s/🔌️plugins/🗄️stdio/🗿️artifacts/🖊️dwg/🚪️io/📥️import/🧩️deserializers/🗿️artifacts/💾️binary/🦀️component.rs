//! 📥️ Deserialize `stdio.dwg` from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::dwg::DwgSnapshot;

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<DwgSnapshot, store::PackError> {
    crate::artifacts::dwg::schema::snapshot::decode_dwg(&from.bytes).map_err(|e| store::PackError::Schema(e))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<DwgSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
