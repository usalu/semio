use crate::RemodelingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Confidence, Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
/// 🎯️ The foreign dialect this leaf reads.
pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 🧩️ `s.stdio.txt@utf-8/*` → `s.remodel.remodeling@1/*` — this subset's own DSL reader, the exact
/// inverse of the export leaf's `print_dsl`.
pub struct TxtIntoRemodeling;

impl Deserializer<RemodelingSnapshot> for TxtIntoRemodeling {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn sniff(payload: &IoPayload) -> Confidence {
        match payload {
            IoPayload::Text(text) if text.contains("remodeling") => Confidence::Low,
            _ => Confidence::None,
        }
    }
    async fn deserialize(payload: &IoPayload) -> IoResult<RemodelingSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "txt→remodeling: expected a text payload".to_string(), diagnostics: Vec::new() });
        };
        let snapshot = <RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError { message: format!("txt→remodeling: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
