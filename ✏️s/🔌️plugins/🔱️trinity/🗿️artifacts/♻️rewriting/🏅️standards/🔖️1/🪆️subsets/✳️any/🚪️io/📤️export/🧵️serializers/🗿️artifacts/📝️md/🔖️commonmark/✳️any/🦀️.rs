//! rewriting -> md
use crate::artifacts::rewriting::RewritingSnapshot;
use semio_s_plugin_stdio::artifacts::md::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

/// ✒️ Wraps the printed DSL text as md's own block model (see `engine::parse_markdown_blocks`) --
/// `MdSnapshot` is a typed block tree, not a raw `body: String`, since the schema overhaul.
pub fn register() {}

pub fn serialize(snapshot: &RewritingSnapshot) -> Result<MdSnapshot, store::TextError> {
    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: parse_markdown_blocks(&<RewritingSnapshot as store::ArtifactDsl>::print_dsl(snapshot)) })
}

pub fn serialize_bytes(snapshot: &RewritingSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<RewritingSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
