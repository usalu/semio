//! 🚪️ curate <- zip — foreign `Deserializer<CurateSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3).
//!
//! ⚠️ Pre-existing, not fixed this pass: `ZipSnapshot`'s own shape (`schema`/`entries`/`comment`,
//! a zip archive) shares no fields with `CurateSnapshot` (`catalog`/`stock_extra`/`curated`, a
//! catalogue). The naive `serde_json` structural bridge below therefore always fails for real zip
//! content — confirmed by inspection, not a regression introduced by this pass. No domain-correct
//! "zip archive -> curate catalogue" mapping is defined anywhere in this codebase, so `deserialize`
//! is preserved byte-for-byte and honestly labeled `IoFidelity::Lossy` rather than silently claiming
//! a working conversion. See `📓️w4-sourcing-report.md` `## openQuestions`.
use crate::artifacts::curate::CurateSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub async fn deserialize(from: &ZipSnapshot) -> Result<CurateSnapshot, store::TextError> {
    let _ = STDIO_ZIP_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("curate<-zip: {e}"), dsl::TextSpan::at(1, 1)))
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<CurateSnapshot, store::TextError> {
    let wire = <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    deserialize(&wire)
}

pub struct ZipIntoCurate;

impl Deserializer<CurateSnapshot> for ZipIntoCurate {
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<CurateSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "ZipIntoCurate: expected a binary zip payload".to_string(), diagnostics: Vec::new() });
        };
        deserialize_bytes(bytes).map(IoOutcome::clean).map_err(|error| IoError { message: format!("ZipIntoCurate: {error}"), diagnostics: Vec::new() })
    }
}
