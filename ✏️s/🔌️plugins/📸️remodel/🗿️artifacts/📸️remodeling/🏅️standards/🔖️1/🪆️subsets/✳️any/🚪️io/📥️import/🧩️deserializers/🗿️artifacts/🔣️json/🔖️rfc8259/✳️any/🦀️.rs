use crate::RemodelingSnapshot;
use crate::REMODELING_DOCUMENT_SCHEMA;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework::io_schema::Confidence,semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf reads.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ Parses rfc8259 text into this subset's snapshot. An absent/empty `schema` is filled with
/// `REMODELING_DOCUMENT_SCHEMA` so a hand-authored json is still accepted.
pub fn from_json_text(text: &str) -> Result<RemodelingSnapshot, IoError> {
    let value = parse_json_text(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json→remodeling: parse failed: {error}"))))?;
    let raw = crate::standards::v1::subsets::any::io::remodeling_json::convert(semio_framework_pack_json::to_dsl_value(&JsonSnapshot::from_value(value).to_pack_value()),true).map_err(|message|IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)))?;
    let mut snapshot: RemodelingSnapshot = semio_framework_value::FromValue::from_value(raw).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("json→remodeling: {error}"))))?;
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
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "json→remodeling: expected a text json payload".to_string())));
        };
        Ok(IoOutcome::clean(from_json_text(text)?))
    }
}
