//! 🚪️ dag <- png — foreign `Deserializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). A raster image has no
//! node/edge/graph concept, so this is a best-effort structural reinterpretation via
//! `serde_json` — succeeds only for a `PngSnapshot` whose serialized shape happens to already
//! match `DagSnapshot`'s own, `IoFidelity::Lossy`.

use crate::artifacts::dag::DagSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub fn deserialize(from: &PngSnapshot) -> Result<DagSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("dag<-png: {e}"), dsl::TextSpan::at(1, 1)))
}

pub struct PngIntoDag;

impl Deserializer<DagSnapshot> for PngIntoDag {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<DagSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PngIntoDag: expected a binary png payload".to_string(), diagnostics: Vec::new() });
        };
        let png = <PngSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("PngIntoDag: png decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = deserialize(&png).map_err(|error| IoError { message: format!("PngIntoDag: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
