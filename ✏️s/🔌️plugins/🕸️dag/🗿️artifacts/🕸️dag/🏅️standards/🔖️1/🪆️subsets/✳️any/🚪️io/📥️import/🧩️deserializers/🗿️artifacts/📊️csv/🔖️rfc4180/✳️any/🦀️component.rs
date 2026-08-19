//! 🚪️ dag <- csv — foreign `Deserializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). A flat grid has no
//! node/edge/graph concept, so this is a best-effort structural reinterpretation via
//! `serde_json` — succeeds only for a `CsvSnapshot` whose serialized shape happens to already
//! match `DagSnapshot`'s own, `IoFidelity::Lossy`.

use crate::artifacts::dag::DagSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub async fn deserialize(from: &CsvSnapshot) -> Result<DagSnapshot, store::TextError> {
    let _ = STDIO_CSV_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("dag<-csv: {e}"), dsl::TextSpan::at(1, 1)))
}

pub struct CsvIntoDag;

impl Deserializer<DagSnapshot> for CsvIntoDag {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<DagSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "CsvIntoDag: expected a binary csv payload".to_string(), diagnostics: Vec::new() });
        };
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("CsvIntoDag: csv decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = deserialize(&csv).map_err(|error| IoError { message: format!("CsvIntoDag: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
