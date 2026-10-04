//! deser step via binary
use crate::StepSnapshot;
use semio_s_artifact_stdio_binary::BinarySnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<StepSnapshot, store::PackError> {
    let text = String::from_utf8(from.bytes.clone()).map_err(|e| store::PackError::from(semio_framework_value::ValueError::from(e)))?;
    let document = semio_s_artifact_stdio_contract::part21::parse_part21(&text).map_err(|error| store::PackError::from(semio_framework_value::ValueError::from(error)))?;
    Ok(StepSnapshot::from_part21_document(&document))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<StepSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
