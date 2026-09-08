//! home <- json
use crate::SHomeSnapshot;
use crate::S_HOME_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<SHomeSnapshot, store::TextError> {
    let _ = S_HOME_DOCUMENT_SCHEMA;
    let mut out: SHomeSnapshot = dsl::FromValue::from_value(pack::json_to_dsl_value(&from.to_pack_value())).map_err(|e: dsl::ValueError| store::TextError::new(format!("home<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = S_HOME_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<SHomeSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value: pack::JsonValue = pack::parse_json(text).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}
