//! Deserialize stdio.jpg from stdio.binary.

use crate::{JpgSnapshot, STDIO_JPG_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_binary::BinarySnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize(from: &BinarySnapshot) -> Result<JpgSnapshot, store::PackError> {
    let mut snap = crate::standards::v_jfif_1_01::subsets::document::io::decode_jpg(&from.bytes).map_err(|e| store::PackError::from(semio_framework_value::ValueError::from(e)))?;
    snap.schema = STDIO_JPG_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn deserialize_bytes(bytes: &[u8]) -> Result<JpgSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
