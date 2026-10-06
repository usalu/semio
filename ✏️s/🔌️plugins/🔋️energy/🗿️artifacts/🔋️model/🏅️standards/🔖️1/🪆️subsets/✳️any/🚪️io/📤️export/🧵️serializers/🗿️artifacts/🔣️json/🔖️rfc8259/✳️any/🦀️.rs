//! model -> json
use crate::EnergyModelSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::{write_json_pretty};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::{parse_json_text};
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

/// 🌉 Bridges via this crate's `ToValue` impl through `pack::json::to_json_string`, then back into
/// json's own RFC8259 text codec (`JsonSnapshot::value` is `JsonValue`, json's own
/// key-order/lexeme-preserving model). `serde` is not reachable here: `EnergyModelSnapshot` carries
/// `store::ArtifactChild<S>` and `store::ArtifactLink`, and neither derives `Serialize`
/// (`🏪️store/🦀️.rs:2996` — `ToValue`/`FromValue` only).
pub async fn register() {}

pub fn serialize(snapshot: &EnergyModelSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    Ok(JsonSnapshot::from_value(parse_json_text(&semio_framework_pack_json::to_json_string(snapshot))?))
}

pub fn serialize_bytes(snapshot: &EnergyModelSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}
