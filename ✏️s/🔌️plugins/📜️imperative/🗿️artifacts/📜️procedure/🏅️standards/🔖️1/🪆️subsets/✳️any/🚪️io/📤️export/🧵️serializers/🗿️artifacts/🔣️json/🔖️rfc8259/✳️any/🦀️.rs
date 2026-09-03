//! imperative -> json
use crate::artifacts::procedure::ProcedureSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🩹️ `stdio_gap` fix (see the paired import leaf's doc comment) — bridges via json's own RFC8259
/// text codec (`JsonSnapshot::value` is `JsonValue`, not `serde_json::Value`), mirroring `🔱️jack`'s
/// own fix. Goes through `ProcedureSnapshot`'s own `ToValue` impl and `pack::json`'s
/// `DslValue`↔`pack::JsonValue` bridge (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// same cross-plugin bridge the `🔱️trinity` batch established), never `serde_json`.
pub fn serialize(snapshot: &ProcedureSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(snapshot));
    Ok(JsonSnapshot::from_value(value))
}

pub fn serialize_bytes(snapshot: &ProcedureSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
