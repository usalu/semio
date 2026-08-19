//! rewrite <- md
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::md::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks;

pub async fn register() {}

pub async fn deserialize(from: &MdSnapshot) -> Result<RewriteSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <RewriteSnapshot as store::ArtifactDsl>::parse_dsl(&render_markdown_blocks(&from.blocks))
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<RewriteSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <RewriteSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
