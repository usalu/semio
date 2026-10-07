use crate::RemodelingSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use semio_framework_value::ToValue;
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🧵️ Closed IEEE word objects and exact decimal integer strings preserve the declared JSON state.
pub struct RemodelingIntoJson;

impl Serializer<RemodelingSnapshot> for RemodelingIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &RemodelingSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let owned = crate::standards::v1::subsets::any::io::remodeling_json::convert(from.to_value(),false).map_err(|message|IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)))?;
        let value = semio_framework_pack_json::from_dsl_value(&owned);
        Ok(IoOutcome::clean(IoPayload::Text(write_json_pretty(&JsonSnapshot::from_value(value).value))))
    }
}
