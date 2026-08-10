//! remodel <- las
use crate::artifacts::remodel::schema::snapshot::RemodelSnapshot;
use semio_s_plugin_stdio::artifacts::las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &LasSnapshot) -> Result<RemodelSnapshot, store::TextError> {
    let _ = STDIO_LAS_DOCUMENT_SCHEMA;
    let bytes = <LasSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RemodelSnapshot, store::TextError> {
    <RemodelSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| {
        <RemodelSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}
