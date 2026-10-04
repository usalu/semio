//! imperative <- json
use crate::ProcedureSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🩹️ `stdio_gap` fix (see the CSV import leaf's doc comment for the wave that caused this) —
/// `JsonSnapshot.value` moved from `serde_json::Value` to stdio's own lexeme-preserving `JsonValue`
/// (`#[value(tag = "kind")]`, an intentional boundary type, not structurally plain JSON); bridges via
/// `JsonSnapshot::to_pack_value` and `ProcedureSnapshot`'s own `FromValue` impl
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS), never `serde_json`.
pub fn deserialize(from: &JsonSnapshot) -> Result<ProcedureSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let dsl_value = semio_framework_pack_json::to_dsl_value(&from.to_pack_value());
    let out: ProcedureSnapshot = semio_framework_value::FromValue::from_value(dsl_value).map_err(|e: semio_framework_value::ValueError| semio_framework_diagnostic::TextError::new(e.kind, format!("imperative<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProcedureSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
