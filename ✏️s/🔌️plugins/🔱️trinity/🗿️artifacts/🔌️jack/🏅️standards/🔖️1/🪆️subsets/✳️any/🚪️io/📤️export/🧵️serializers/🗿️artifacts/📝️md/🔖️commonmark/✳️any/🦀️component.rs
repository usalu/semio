//! jack -> md
use crate::artifacts::jack::JackSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::md::engine::parse_markdown_blocks;

/// ✒️ Wraps the printed DSL text as md's own block model (see `engine::parse_markdown_blocks`) —
/// `MdSnapshot` is a typed block tree, not a raw `body: String`, since the schema overhaul.
pub fn register() {}

pub fn serialize(snapshot: &JackSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        blocks: parse_markdown_blocks(&<JackSnapshot as store::ArtifactDsl>::print_dsl(snapshot)),
    })
}

pub fn serialize_bytes(snapshot: &JackSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<JackSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
