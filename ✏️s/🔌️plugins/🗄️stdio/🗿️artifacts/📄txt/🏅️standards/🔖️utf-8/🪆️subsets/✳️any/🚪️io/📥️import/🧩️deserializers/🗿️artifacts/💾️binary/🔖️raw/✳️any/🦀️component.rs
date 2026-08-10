//! deser txt via binary
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
pub fn register() {}
pub fn deserialize(from: &BinarySnapshot) -> Result<TxtSnapshot, store::PackError> {
    let text = String::from_utf8(from.bytes.clone()).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<TxtSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
