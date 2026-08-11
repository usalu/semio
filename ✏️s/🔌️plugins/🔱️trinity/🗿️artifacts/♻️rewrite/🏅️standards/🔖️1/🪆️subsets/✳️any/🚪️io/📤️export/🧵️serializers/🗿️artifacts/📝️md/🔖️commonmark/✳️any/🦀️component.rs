//! rewrite -> md
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::md::engine::parse_markdown_blocks;

/// ✒️ Wraps the printed DSL text as md's own block model (see `engine::parse_markdown_blocks`) --
/// `MdSnapshot` is a typed block tree, not a raw `body: String`, since the schema overhaul.
pub fn register() {}

pub fn serialize(snapshot: &RewriteSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot {
        schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
        blocks: parse_markdown_blocks(&<RewriteSnapshot as store::ArtifactDsl>::print_dsl(snapshot)),
    })
}

pub fn serialize_bytes(snapshot: &RewriteSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<RewriteSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
