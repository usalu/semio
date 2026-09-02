//! 🚪️ draw <- json — foreign `Deserializer<DrawSnapshot>` (design.md §3). Real: bridges via
//! stdio's own real RFC8259 text codec (`parse_json_text`), not `serde_json::Value`, since
//! `JsonSnapshot.value` is stdio's own lexeme-preserving `JsonValue` model. `IoFidelity::Exact` —
//! `DrawSnapshot`'s own `#[derive(ToValue, FromValue)]` JSON shape round-trips losslessly.

use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoDraw;

impl Deserializer<DrawSnapshot> for JsonIntoDraw {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn deserialize(payload: &IoPayload) -> IoResult<DrawSnapshot> {
        let text = match payload {
            IoPayload::Text(text) => text.clone(),
            IoPayload::Binary(bytes) => std::str::from_utf8(bytes).map_err(|error| IoError { message: format!("JsonIntoDraw: not valid utf-8: {error}"), diagnostics: Vec::new() })?.to_string(),
        };
        let value = parse_json_text(&text).map_err(|error| IoError { message: format!("JsonIntoDraw: {error}"), diagnostics: Vec::new() })?;
        let from = JsonSnapshot::from_value(value);
        let mut snap: DrawSnapshot = dsl::FromValue::from_value(dsl::json::to_dsl_value(&from.to_pack_value())).map_err(|error| IoError { message: format!("JsonIntoDraw: {error}"), diagnostics: Vec::new() })?;
        if snap.schema.is_empty() {
            snap.schema = DRAW_DOCUMENT_SCHEMA.into();
        }
        Ok(IoOutcome::clean(snap))
    }
}
