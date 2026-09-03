//! imperative <- json
use crate::artifacts::procedure::ProcedureSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub fn register() {}

/// 🩹️ `stdio_gap` fix (see the CSV import leaf's doc comment for the wave that caused this) —
/// `JsonSnapshot.value` moved from `serde_json::Value` to stdio's own lexeme-preserving `JsonValue`
/// (`#[value(tag = "kind")]`, an intentional boundary type, not structurally plain JSON); bridges via
/// `JsonSnapshot::to_pack_value` and `ProcedureSnapshot`'s own `FromValue` impl
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS), never `serde_json`.
pub fn deserialize(from: &JsonSnapshot) -> Result<ProcedureSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let dsl_value = dsl::os_pack::json::to_dsl_value(&from.to_pack_value());
    let out: ProcedureSnapshot = dsl::FromValue::from_value(dsl_value).map_err(|e: dsl::ValueError| store::TextError::new(format!("imperative<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProcedureSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
