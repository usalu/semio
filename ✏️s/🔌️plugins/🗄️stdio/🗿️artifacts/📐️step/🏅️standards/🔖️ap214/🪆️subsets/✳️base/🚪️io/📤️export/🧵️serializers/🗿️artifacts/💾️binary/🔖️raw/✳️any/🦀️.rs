//! Serialize stdio.step to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::step::StepSnapshot;

//#region Codec
/// Register serializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// UTF-8 encode text into a BinarySnapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &StepSnapshot) -> BinarySnapshot {
    let text = crate::artifacts::step::engine::part21::write_part21(&from.to_part21_document());
    BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: text.into_bytes() }
}

/// Encode as binary pack bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_bytes(from: &StepSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from), &store::PackEncodeOptions::default())
}
//#endregion Codec
