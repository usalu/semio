//! Serialize dag to stdio.md.
use crate::artifacts::dag::DagSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &DagSnapshot) -> Result<MdSnapshot, store::PackError> {
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body: <DagSnapshot as store::ArtifactDsl>::print_dsl(from) })
}

pub fn serialize_text(from: &DagSnapshot) -> Result<String, store::PackError> {
    Ok(<DagSnapshot as store::ArtifactDsl>::print_dsl(from))
}
