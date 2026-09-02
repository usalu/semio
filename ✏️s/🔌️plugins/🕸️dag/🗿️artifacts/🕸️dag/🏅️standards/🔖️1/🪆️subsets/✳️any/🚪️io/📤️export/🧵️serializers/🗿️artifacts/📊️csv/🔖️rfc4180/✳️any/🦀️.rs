//! 🚪️ dag -> csv — foreign `Serializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling
//! `Deserializer`'s best-effort `serde_json` structural reinterpretation — a flat grid has no
//! node/edge/graph concept, so this hop is `IoFidelity::Lossy`.

use crate::artifacts::dag::DagSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::CsvSnapshot;

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub async fn serialize(from: &DagSnapshot) -> Result<CsvSnapshot, store::PackError> {
    CsvSnapshot::from_value(from.to_value()).map_err(|e| store::PackError::Schema(e.to_string()))
}

pub struct DagIntoCsv;

impl Serializer<DagSnapshot> for DagIntoCsv {
    const INTO: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &DagSnapshot) -> IoResult<IoPayload> {
        let csv = serialize(from).map_err(|error| IoError { message: format!("DagIntoCsv: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<CsvSnapshot as store::ArtifactPack>::encode_pack(&csv))))
    }
}
