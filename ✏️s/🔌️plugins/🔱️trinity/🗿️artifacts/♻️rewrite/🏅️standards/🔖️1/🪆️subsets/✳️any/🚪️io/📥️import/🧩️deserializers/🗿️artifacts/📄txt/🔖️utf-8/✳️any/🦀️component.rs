//! rewrite <- txt
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_s_plugin_stdio::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<RewriteSnapshot, store::TextError> {
    let _ = STDIO_TXT_DOCUMENT_SCHEMA;
    <RewriteSnapshot as store::ArtifactDsl>::parse_dsl(&from.text)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewriteSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <RewriteSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
