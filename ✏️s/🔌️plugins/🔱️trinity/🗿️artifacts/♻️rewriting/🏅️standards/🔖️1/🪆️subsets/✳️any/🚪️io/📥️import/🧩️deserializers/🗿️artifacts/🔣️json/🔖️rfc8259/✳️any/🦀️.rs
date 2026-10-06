//! rewriting <- json
use crate::RewritingSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<RewritingSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = crate::standards::v1::subsets::any::schema::snapshot::json::convert(semio_framework_pack_json::to_dsl_value(&from.to_pack_value()), true).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let out: RewritingSnapshot = semio_framework_value::FromValue::from_value(value).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<RewritingSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
