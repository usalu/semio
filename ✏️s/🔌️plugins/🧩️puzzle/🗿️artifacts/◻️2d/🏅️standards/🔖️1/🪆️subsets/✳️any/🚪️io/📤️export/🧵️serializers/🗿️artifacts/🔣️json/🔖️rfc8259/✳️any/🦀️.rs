//! 📤️ Declared Puzzle2d JSON preserves every typed binary64 word.
use crate::Puzzle2dSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 📜️ The authoritative declared field-wire schema; native snapshot state has a separate schema.
pub const JSON_SCHEMA: &str = crate::standards::v1::subsets::any::io::puzzle2d_json::SCHEMA;

pub fn register() {}

pub fn serialize(snapshot: &Puzzle2dSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let raw = semio_framework_value::ToValue::to_value(snapshot);
    let raw = crate::standards::v1::subsets::any::io::puzzle2d_json::convert(raw, false)
        .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("puzzle2d->json: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&raw)))
}

pub fn serialize_bytes(snapshot: &Puzzle2dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
