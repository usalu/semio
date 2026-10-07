//! 🚪️ block2d ← json — foreign `Deserializer<Block2dSnapshot>` on the framework's `io_mechanism`
//! channel, the exact inverse of the sibling `📤️export` leaf: `IoFidelity::Exact`.

use crate::Block2dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use {semio_framework::io_schema::Confidence,semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot — also used by the `🎒️zip` container leaf.
/// 📸️ Present schema strings, including the empty string, are retained exactly.
pub fn from_json_text(text: &str) -> Result<Block2dSnapshot, IoError> {
    crate::standards::v1::subsets::any::io::json::from_json_text(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json→block2d: {error}"))))
}

/// 🧩️ `s.stdio.json@rfc8259/*` → `s.block.block2d@1/*`.
pub struct JsonIntoBlock2d;

impl Deserializer<Block2dSnapshot> for JsonIntoBlock2d {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.trim_start().starts_with('{') => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<Block2dSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "json→block2d: expected a text json payload".to_string())));
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
