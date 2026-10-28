//! 🧮️ External IFC4 Part21 bytes enter through the declared exchange parser.
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_s_artifact_stdio_binary::BinarySnapshot;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<IfcSnapshot, store::PackError> {
    let document = crate::standards::v4::subsets::any::io::decode_ifc4_document(&from.bytes).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))?;
    Ok(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::from_part21_document(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::STDIO_IFC_DOCUMENT_SCHEMA, &document))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<IfcSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?)
}
