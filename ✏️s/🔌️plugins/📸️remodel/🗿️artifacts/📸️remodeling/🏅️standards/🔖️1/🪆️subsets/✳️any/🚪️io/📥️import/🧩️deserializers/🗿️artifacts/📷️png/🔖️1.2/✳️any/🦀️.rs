//! remodeling <- png
use crate::artifacts::remodeling::schema::snapshot::RemodelingSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub async fn register() {}

pub async fn deserialize(from: &PngSnapshot) -> Result<RemodelingSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let bytes = <PngSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<RemodelingSnapshot, store::TextError> {
    <RemodelingSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| <RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes)))
}
