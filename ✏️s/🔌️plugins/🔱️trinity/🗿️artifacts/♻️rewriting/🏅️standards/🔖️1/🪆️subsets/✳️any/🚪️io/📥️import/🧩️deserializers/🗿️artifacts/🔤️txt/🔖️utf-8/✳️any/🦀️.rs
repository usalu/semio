//! rewriting <- txt
use crate::artifacts::rewriting::RewritingSnapshot;
use semio_s_plugin_stdio::artifacts::txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<RewritingSnapshot, store::TextError> {
    let _ = STDIO_TXT_DOCUMENT_SCHEMA;
    <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewritingSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
