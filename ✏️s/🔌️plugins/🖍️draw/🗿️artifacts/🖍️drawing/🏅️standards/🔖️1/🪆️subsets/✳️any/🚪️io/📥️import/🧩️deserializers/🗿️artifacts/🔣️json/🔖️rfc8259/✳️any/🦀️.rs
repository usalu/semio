//! 🚪️ drawing <- json — foreign `Deserializer<DrawingSnapshot>` (design.md §3). Real: bridges via
//! stdio's own real RFC8259 text codec (`parse_json_text`), not `serde_json::Value`, since
//! `JsonSnapshot.value` is stdio's own lexeme-preserving `JsonValue` model. `IoFidelity::Exact` —
//! `DrawingSnapshot`'s own `#[derive(ToValue, FromValue)]` JSON shape round-trips losslessly.

use crate::{DrawingSnapshot, DRAWING_DOCUMENT_SCHEMA};
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoDraw;

impl Deserializer<DrawingSnapshot> for JsonIntoDraw {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<DrawingSnapshot> {
        let text = match payload {
            IoPayload::Text(text) => text.clone(),
            IoPayload::Binary(bytes) => std::str::from_utf8(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("JsonIntoDraw: not valid utf-8: {error}"))))?.to_string(),
        };
        let value = parse_json_text(&text).map_err(json_parse_error)?;
        let from = JsonSnapshot::from_value(value);
        let mut snap: DrawingSnapshot = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&from.to_pack_value())).map_err(|error: semio_framework_value::ValueError| IoError::from_value_error(error.under("JsonIntoDraw")))?;
        if snap.schema.is_empty() {
            snap.schema = DRAWING_DOCUMENT_SCHEMA.into();
        }
        Ok(IoOutcome::clean(snap))
    }
}

/// 📍️ Retains the authored JSON source position and semantic refusal kind.
fn json_parse_error(error: semio_framework_diagnostic::TextError) -> IoError {
    use semio_framework_diagnostic::{Diagnostic,ExpectedSet,FaultCode,FaultScope,Severity};
    IoError {
        cause: semio_framework_value::ValueError::new(error.kind,format!("JsonIntoDraw: {}",error.message)),
        diagnostics: vec![Diagnostic {
            code: FaultCode::new("draw.json.parse"), severity: Severity::Error, span: error.span,
            message: error.message, expected: error.expected.map(|token|ExpectedSet {tokens:vec![token],keywords:Vec::new(),keys:Vec::new()}), scope:FaultScope::default(),
        }],
    }
}
