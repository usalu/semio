//! Deserialize dag via stdio.md.
use crate::artifacts::dag::DagSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &MdSnapshot) -> Result<DagSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_text())
}

pub fn deserialize_text(text: &str) -> Result<DagSnapshot, store::TextError> {
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
