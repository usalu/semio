//! Serialize stdio.step to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::step::StepSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// UTF-8 encode text into a BinarySnapshot.
pub fn serialize(from: &StepSnapshot) -> BinarySnapshot {
    let text = crate::artifacts::step::schema::snapshot::step_brep_to_text(&from.brep);
    BinarySnapshot {
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        bytes: text.into_bytes(),
    }
}

/// Encode as binary pack bytes.
pub fn serialize_bytes(from: &StepSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from), &store::PackEncodeOptions::default())
}
//#endregion Codec
