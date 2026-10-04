//! rewriting <- txt
use crate::RewritingSnapshot;
use semio_s_artifact_stdio_txt::{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<RewritingSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_TXT_DOCUMENT_SCHEMA;
    <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewritingSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}
