//! deser step via binary
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::step::StepSnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<StepSnapshot, store::PackError> {
    let text = String::from_utf8(from.bytes.clone()).map_err(|e| store::PackError::Schema(e.to_string()))?;
    <StepSnapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|e| store::PackError::Schema(e.to_string()))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<StepSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
