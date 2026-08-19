//! 🚪️ draw -> json — foreign `Serializer<DrawSnapshot>` (design.md §3). Real: bridges via
//! stdio's own real RFC8259 text writer (`write_json_pretty`), not `serde_json::to_string`, since
//! `JsonSnapshot.value` is stdio's own lexeme-preserving `JsonValue` model. `IoFidelity::Exact`.

use crate::artifacts::draw::DrawSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct DrawIntoJson;

impl Serializer<DrawSnapshot> for DrawIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &DrawSnapshot) -> IoResult<IoPayload> {
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("DrawIntoJson: {error}"), diagnostics: Vec::new() })?;
        let json = JsonSnapshot::from_value(value);
        Ok(IoOutcome::clean(IoPayload::Text(write_json_pretty(&json.value))))
    }
}
