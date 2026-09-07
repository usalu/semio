use crate::artifacts::remodeling::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_os_kernel::ToValue;
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🧵️ `s.remodel.remodeling@1/*` → `s.stdio.json@rfc8259/*`. The scene is a pure record tree, so its
/// `dsl::ToValue` projection is total: every field survives, and the sibling import leaf reverses it
/// exactly — the one `IoFidelity::Exact` binary-free hop this subset owns.
pub struct RemodelingIntoJson;

impl Serializer<RemodelingSnapshot> for RemodelingIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let value = pack::json::from_dsl_value(&from.to_value());
        Ok(IoOutcome::clean(IoPayload::Text(write_json_pretty(&JsonSnapshot::from_value(value).value))))
    }
}
