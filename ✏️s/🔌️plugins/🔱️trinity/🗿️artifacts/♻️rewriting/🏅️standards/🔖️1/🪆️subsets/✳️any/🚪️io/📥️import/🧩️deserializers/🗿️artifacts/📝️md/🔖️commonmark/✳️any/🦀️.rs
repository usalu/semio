//! rewriting <- md
use crate::artifacts::rewriting::RewritingSnapshot;
use semio_s_plugin_stdio::artifacts::md::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &MdSnapshot) -> Result<RewritingSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(&render_markdown_blocks(&from.blocks))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewritingSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
