//! imperative -> json
use crate::ProcedureSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🩹️ `stdio_gap` fix (see the paired import leaf's doc comment) — bridges via json's own RFC8259
/// text codec (`JsonSnapshot::value` is `JsonValue`, not `serde_json::Value`), mirroring `🔱️jack`'s
/// own fix. Goes through `ProcedureSnapshot`'s own `ToValue` impl and `pack::json`'s
/// `DslValue`↔`pack::JsonValue` bridge (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// same cross-plugin bridge the `🔱️trinity` batch established), never `serde_json`.
pub fn serialize(snapshot: &ProcedureSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &ProcedureSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
