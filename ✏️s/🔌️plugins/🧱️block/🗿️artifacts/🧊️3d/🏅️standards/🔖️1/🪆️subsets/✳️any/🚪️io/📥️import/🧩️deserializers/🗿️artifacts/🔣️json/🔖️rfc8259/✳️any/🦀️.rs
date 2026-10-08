//! 🚪️ block3d ← json — foreign `Deserializer<Block3dSnapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf: `IoFidelity::Exact`.

use crate::Block3dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework::io_schema::Confidence,semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot — also used by the `🎒️zip` container leaf.
/// 📸️ Present schema strings, including the empty string, are retained exactly.
pub fn from_json_text(text: &str) -> Result<Block3dSnapshot, IoError> {
    let value = parse_json_text(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json→block3d: parse failed: {error}"))))?;
    // 🎯️ Through stdio's own first-party `to_pack_value()` bridge, never `to_serde_value()`:
    // `serde_json`'s default (no `float_roundtrip`) number parser rebuilds an f64 as
    // `significand as f64 * 10^exponent`, off by one ULP for any 17-significant-digit literal
    // (`0.42839899821678995` came back as `0.4283989982167899`), which broke this leaf's own
    // `IoFidelity::Exact` claim for every example carrying full-precision geometry.
    let mut raw: semio_framework_value::DslValue = semio_framework_pack_json::to_dsl_value(&JsonSnapshot::from_value(value).to_pack_value());
    crate::standards::v1::subsets::any::schema::snapshot::json_transport::convert(&mut raw, true).map_err(IoError::from_value_error)?;
    let snapshot: Block3dSnapshot = semio_framework_value::FromValue::from_value(raw).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json→block3d: {error}"))))?;
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
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "json→block3d: expected a text json payload".to_string())));
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
