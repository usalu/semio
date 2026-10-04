//! home <- json
use crate::SHomeSnapshot;
use crate::S_HOME_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<SHomeSnapshot, semio_framework_diagnostic::TextError> {
    let _ = S_HOME_DOCUMENT_SCHEMA;
    let mut out: SHomeSnapshot = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&from.to_pack_value())).map_err(|e: semio_framework_value::ValueError| semio_framework_diagnostic::TextError::new(e.kind, format!("home<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = S_HOME_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<SHomeSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value: semio_framework_pack_json::Value = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e.into_value_error(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}
