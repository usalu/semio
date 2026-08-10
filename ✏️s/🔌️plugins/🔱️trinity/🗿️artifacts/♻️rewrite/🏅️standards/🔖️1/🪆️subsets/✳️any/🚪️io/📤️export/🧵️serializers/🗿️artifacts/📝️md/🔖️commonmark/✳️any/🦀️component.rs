//! rewrite -> md
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(snapshot: &RewriteSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        body: <RewriteSnapshot as store::ArtifactDsl>::print_dsl(snapshot),
    })
}

pub fn serialize_bytes(snapshot: &RewriteSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(serialize(snapshot)?.body.into_bytes())
}
