//! Serialize stdio.zip to stdio.deflate (encode ZIP then zlib-compress).

use crate::artifacts::deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};
use crate::artifacts::zip::ZipSnapshot;

//#region Codec
/// Register serializer hooks.
pub fn register() {}

/// Encode ZIP bytes as the deflate artifact's typed payload (real zlib compression happens on
/// `ArtifactPack`/`ArtifactDsl` encode, via `engine::encode_deflate_snapshot`).
pub fn serialize(from: &ZipSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let zip_bytes = crate::artifacts::zip::standards::v2_0::subsets::any::io::encode_zip(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: crate::artifacts::deflate::schema::snapshot::DeflateLevelHint::default(), dict_id: None, payload: zip_bytes })
}

/// Encode as deflate pack bytes.
pub fn serialize_bytes(from: &ZipSnapshot) -> Result<Vec<u8>, store::PackError> {
    store::ArtifactPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
