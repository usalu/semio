//! 🚪️ block5d ← json — foreign `Deserializer<Block5dSnapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf: `IoFidelity::Exact`.

use crate::artifacts::block5d::{Block5dSnapshot, BLOCK_5D_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot — also used by the `🎒️zip` container leaf.
/// An absent/empty `schema` is filled with `BLOCK_5D_SCHEMA` so a hand-authored json is still accepted.
pub fn from_json_text(text: &str) -> Result<Block5dSnapshot, IoError> {
    let value = parse_json_text(text).map_err(|error| IoError { message: format!("json→block5d: parse failed: {error}"), diagnostics: Vec::new() })?;
    let raw: dsl::DslValue = JsonSnapshot::from_value(value).to_serde_value().into();
    let mut snapshot: Block5dSnapshot = dsl::FromValue::from_value(raw).map_err(|error| IoError { message: format!("json→block5d: {error}"), diagnostics: Vec::new() })?;
    if snapshot.schema.is_empty() {
        snapshot.schema = BLOCK_5D_SCHEMA.to_string();
    }
    Ok(snapshot)
}

/// 🧩️ `s.stdio.json@rfc8259/*` → `s.block.block5d@1/*`.
pub struct JsonIntoBlock5d;

impl Deserializer<Block5dSnapshot> for JsonIntoBlock5d {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.trim_start().starts_with('{') => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Block5dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "json→block5d: expected a text json payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
