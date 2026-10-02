use crate::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_os_kernel::ToValue;
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_json::schema::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🧵️ Closed IEEE word objects and exact decimal integer strings preserve the declared JSON state.
pub struct RemodelingIntoJson;

impl Serializer<RemodelingSnapshot> for RemodelingIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &RemodelingSnapshot) -> IoResult<IoPayload> {
        let owned = crate::standards::v1::subsets::any::io::remodeling_json::convert(from.to_value(),false).map_err(|message|IoError{message,diagnostics:Vec::new()})?;
        let value = pack::json::from_dsl_value(&owned);
        Ok(IoOutcome::clean(IoPayload::Text(write_json_pretty(&JsonSnapshot::from_value(value).value))))
    }
}
