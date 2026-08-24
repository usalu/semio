//! 🚪️ curate <- json — foreign `Deserializer<CurateSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3). Bridges via json's own text codec (`parse_json_text`)
//! then a genuine `serde_json::Value -> CurateSnapshot` structural deserialize — a real, lossless
//! round trip of the same type's own serde shape, so this hop is `IoFidelity::Exact` (unlike the
//! sibling zip/png/stl/obj leaves, which are pre-existing non-functional stubs — see this subset's
//! `📓️w4-sourcing-report.md` `## openQuestions`).
//!
//! `JsonSnapshot::value` is stdio's own `JsonValue` (key-order/lexeme-preserving RFC8259 model,
//! never `serde_json::Value` — see that snapshot module's own doc). Bridges via json's own text
//! codec rather than a per-leaf structural converter, mirroring `s/plugin/lowpoly`'s identical leaf.
use crate::artifacts::curate::CurateSnapshot;
use crate::artifacts::curate::SOURCING_CURATE_SCHEMA;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub fn deserialize(from: &JsonSnapshot) -> Result<CurateSnapshot, store::TextError> {
    let _ = SOURCING_CURATE_SCHEMA;
    let out: CurateSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("curate<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<CurateSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}

pub struct JsonIntoCurate;

impl Deserializer<CurateSnapshot> for JsonIntoCurate {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<CurateSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "JsonIntoCurate: expected a text json payload".to_string(), diagnostics: Vec::new() });
        };
        deserialize_bytes(text.as_bytes()).map(IoOutcome::clean).map_err(|error| IoError { message: format!("JsonIntoCurate: {error}"), diagnostics: Vec::new() })
    }
}
