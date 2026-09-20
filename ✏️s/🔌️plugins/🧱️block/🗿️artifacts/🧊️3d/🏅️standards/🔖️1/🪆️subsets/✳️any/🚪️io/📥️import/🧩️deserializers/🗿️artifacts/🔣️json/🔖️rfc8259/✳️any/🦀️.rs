//! 🚪️ block3d ← json — foreign `Deserializer<Block3dSnapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf: `IoFidelity::Exact`.

use crate::{Block3dSnapshot, BLOCK_3D_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_json::schema::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot — also used by the `🎒️zip` container leaf.
/// An absent/empty `schema` is filled with `BLOCK_3D_SCHEMA` so a hand-authored json is still accepted.
pub fn from_json_text(text: &str) -> Result<Block3dSnapshot, IoError> {
    let value = parse_json_text(text).map_err(|error| IoError { message: format!("json→block3d: parse failed: {error}"), diagnostics: Vec::new() })?;
    // 🎯️ Through stdio's own first-party `to_pack_value()` bridge, never `to_serde_value()`:
    // `serde_json`'s default (no `float_roundtrip`) number parser rebuilds an f64 as
    // `significand as f64 * 10^exponent`, off by one ULP for any 17-significant-digit literal
    // (`0.42839899821678995` came back as `0.4283989982167899`), which broke this leaf's own
    // `IoFidelity::Exact` claim for every example carrying full-precision geometry.
    let raw: dsl::DslValue = dsl::json::to_dsl_value(&JsonSnapshot::from_value(value).to_pack_value());
    let mut snapshot: Block3dSnapshot = dsl::FromValue::from_value(raw).map_err(|error| IoError { message: format!("json→block3d: {error}"), diagnostics: Vec::new() })?;
    if snapshot.schema.is_empty() {
        snapshot.schema = BLOCK_3D_SCHEMA.to_string();
    }
    Ok(snapshot)
}

/// 🧩️ `s.stdio.json@rfc8259/*` → `s.block.block3d@1/*`.
pub struct JsonIntoBlock3d;

impl Deserializer<Block3dSnapshot> for JsonIntoBlock3d {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.trim_start().starts_with('{') => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Block3dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "json→block3d: expected a text json payload".to_string(), diagnostics: Vec::new() });
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
