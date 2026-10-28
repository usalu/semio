//! 📤️ Serialize stdio.ifc to stdio.binary.

use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_s_artifact_stdio_binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

//#region Codec
/// 💾️ Register serializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🖨️ UTF-8 encode text into a BinarySnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &IfcSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::standards::v4::subsets::any::io::encode_ifc4_document(&semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::to_part21_document(from)).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error)))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}

/// ✍️ Encode as binary pack bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_bytes(from: &IfcSnapshot) -> Result<Vec<u8>, store::PackError> {
    pack::snapshot::ArtifactPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
