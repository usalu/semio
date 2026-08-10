//! Serialize stdio.ifc to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::ifc::IfcSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// UTF-8 encode text into a BinarySnapshot.
pub fn serialize(from: &IfcSnapshot) -> BinarySnapshot {
    let text = crate::artifacts::ifc::schema::snapshot::ifc_brep_to_text(&from.brep);
    BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes: text.into_bytes(),
    }
}

/// Encode as binary pack bytes.
pub fn serialize_bytes(from: &IfcSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::DocumentPack::encode_pack_with(&serialize(from), &store::PackEncodeOptions::default())
}
//#endregion Codec
