//! home -> json
use crate::SHomeSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🌉 `JsonSnapshot::value` is stdio's own key-order/lexeme-preserving `JsonValue`, not
/// `pack::JsonValue` directly (stdio's RFC8259 rework) — bridge via `JsonSnapshot::from_value`,
/// which now accepts `pack::JsonValue` too (`impl From<pack::JsonValue> for JsonValue`, stdio's
/// own snapshot component).
pub fn serialize(snapshot: &SHomeSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &SHomeSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
