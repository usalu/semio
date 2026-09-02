//! 🚪️ dag -> json — foreign `Serializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Direct `serde_json`
//! serialization of every field, so this hop is `IoFidelity::Exact`.

use crate::artifacts::dag::DagSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🖨️ Typed encode of `DagSnapshot` into a `JsonSnapshot`'s free-form `value`.
pub async fn serialize(from: &DagSnapshot) -> Result<JsonSnapshot, store::PackError> {
    Ok(JsonSnapshot::from_value(dsl::json::from_dsl_value(&from.to_value())))
}

pub struct DagIntoJson;

impl Serializer<DagSnapshot> for DagIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn serialize(from: &DagSnapshot) -> IoResult<IoPayload> {
        let json = serialize(from).map_err(|error| IoError { message: format!("DagIntoJson: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<JsonSnapshot as store::ArtifactPack>::encode_pack(&json))))
    }
}
