//! 🚪️ curation <- txt — foreign `Deserializer<CurationSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3).
//!
//! 🐛️ Pre-migration content here referenced `crate::artifacts::json`/`crate::artifacts::txt`,
//! types that don't exist in this crate (dead code, never mounted by the old glue, never
//! compiled) -- likely a copy-paste of stdio's own internal json<-txt bridge into the wrong
//! plugin's txt target folder. Left as an honest stub producing this artifact's own real
//! snapshot type, pending a real txt import/export implementation. Wired as a real (if
//! always-failing) `IoEntry` row, `IoFidelity::Lossy`, rather than a dead composer-table entry.
use crate::artifacts::curation::CurationSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub fn deserialize(_from: &semio_s_plugin_stdio::artifacts::txt::TxtSnapshot) -> Result<CurationSnapshot, String> {
    Err("txt import not yet implemented".into())
}
pub fn deserialize_bytes(_bytes: &[u8]) -> Result<CurationSnapshot, String> {
    Err("txt import not yet implemented".into())
}

pub struct TxtIntoCuration;

impl Deserializer<CurationSnapshot> for TxtIntoCuration {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<CurationSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError { message: "TxtIntoCuration: expected a text txt payload".to_string(), diagnostics: Vec::new() });
        };
        deserialize_bytes(text.as_bytes()).map(IoOutcome::clean).map_err(|error| IoError { message: format!("TxtIntoCuration: {error}"), diagnostics: Vec::new() })
    }
}
