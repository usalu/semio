//! 🧾️ model <- json
use crate::snapshot::EnergyModelSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::{write_json_pretty};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::{parse_json_text};
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's
/// own key-order/lexeme-preserving model) and the native receiver, through `pack::json::from_json_str`
/// into the canonical document value. `serde` is not reachable here: `EnergyModelSnapshot` carries
/// `store::ArtifactLink` and persistent owners, and neither derives `Deserialize`.
pub async fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<EnergyModelSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let refused = |e: semio_framework_value::ValueError| semio_framework_diagnostic::TextError::new(e.kind, format!("energy_model<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1));
    let document = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&write_json_pretty(&from.value), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(refused)?;
    EnergyModelSnapshot::from_document(document).map_err(refused)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<EnergyModelSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
