//! Deserialize stdio.tiff from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::tiff::{TiffSnapshot, STDIO_TIFF_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<TiffSnapshot, store::PackError> {
    let mut snap = crate::artifacts::tiff::engine::decode_tiff(&from.bytes).map_err(|e| store::PackError::Schema(e))?;
    snap.schema = STDIO_TIFF_DOCUMENT_SCHEMA.into();
    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<TiffSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
