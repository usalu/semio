//! 🧮️ External IFC4 Part21 bytes enter through the declared exchange parser.
use crate::IfcSnapshot;
use semio_s_artifact_stdio_binary::BinarySnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<IfcSnapshot, store::PackError> {
    let text = std::str::from_utf8(&from.bytes).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))?;
    let document = semio_s_artifact_stdio_contract::part21::parse_part21(text).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))?;
    Ok(crate::schema::snapshot::from_part21_document(crate::STDIO_IFC_DOCUMENT_SCHEMA, &document))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<IfcSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
