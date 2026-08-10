//! jack -> md
use crate::artifacts::jack::JackSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &JackSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <JackSnapshot as store::ArtifactDsl>::print_dsl(snapshot),
    })
}

pub fn serialize_bytes(snapshot: &JackSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.body.into_bytes())
}
