use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::artifacts::remodeling::REMODELING_DOCUMENT_SCHEMA;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot. An absent/empty `schema` is filled with
/// `REMODELING_DOCUMENT_SCHEMA` so a hand-authored json is still accepted.
pub fn from_json_text(text: &str) -> Result<RemodelingSnapshot, IoError> {
    let value = parse_json_text(text).map_err(|error| IoError { message: format!("json→remodeling: parse failed: {error}"), diagnostics: Vec::new() })?;
    let raw: dsl::DslValue = pack::json::to_dsl_value(&JsonSnapshot::from_value(value).to_pack_value());
    let mut snapshot: RemodelingSnapshot = dsl::FromValue::from_value(raw).map_err(|error| IoError { message: format!("json→remodeling: {error}"), diagnostics: Vec::new() })?;
    if snapshot.schema.is_empty() {
        snapshot.schema = REMODELING_DOCUMENT_SCHEMA.to_string();
    }
    Ok(snapshot)
}

/// 🧩️ `s.stdio.json@rfc8259/*` → `s.remodel.remodeling@1/*`, the exact inverse of the export leaf.
pub struct JsonIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for JsonIntoRemodeling {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.trim_start().starts_with('{') => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "json→remodeling: expected a text json payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
