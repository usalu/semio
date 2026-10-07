//! 🚪️ dag -> json — foreign `Serializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Direct `serde_json`
//! serialization of every field, so this hop is `IoFidelity::Exact`.

use crate::DagSnapshot;
use semio_framework_value::ToValue;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🖨️ Typed encode of `DagSnapshot` into a `JsonSnapshot`'s free-form `value`.
pub fn serialize(from: &DagSnapshot) -> Result<JsonSnapshot, store::PackError> {
    Ok(JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&from.to_value())))
}

pub struct DagIntoJson;

impl Serializer<DagSnapshot> for DagIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &DagSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let json = serialize(from).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DagIntoJson: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Binary(<JsonSnapshot as store::ArtifactPack>::encode_pack(&json))))
    }
}
