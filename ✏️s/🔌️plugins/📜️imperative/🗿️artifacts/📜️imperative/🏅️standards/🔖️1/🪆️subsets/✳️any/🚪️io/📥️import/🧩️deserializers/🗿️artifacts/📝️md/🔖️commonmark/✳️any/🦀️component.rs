//! imperative <- md
use crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &MdSnapshot) -> Result<ImperativeSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(&from.body)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ImperativeSnapshot, store::TextError> {
    let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&md)
}
