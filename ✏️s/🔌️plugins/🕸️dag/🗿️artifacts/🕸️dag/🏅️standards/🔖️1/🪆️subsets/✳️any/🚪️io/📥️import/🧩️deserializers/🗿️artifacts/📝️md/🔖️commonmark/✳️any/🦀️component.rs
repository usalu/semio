//! 🚪️ dag <- md — foreign `Deserializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). MD's own text is parsed into
//! blocks (`MdSnapshot::to_text` renders them back) then re-parsed as this plugin's `.dag` DSL —
//! lossless only for text that already round-trips through `stdio.md`'s block model, so this hop
//! is `IoFidelity::Canonical`, not `Exact`.

use crate::artifacts::dag::DagSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub async fn deserialize(from: &MdSnapshot) -> Result<DagSnapshot, store::TextError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_text())
}

pub struct MdIntoDag;

impl Deserializer<DagSnapshot> for MdIntoDag {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn deserialize(payload: &IoPayload) -> IoResult<DagSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "MdIntoDag: expected a binary md payload".to_string(), diagnostics: Vec::new() });
        };
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("MdIntoDag: md decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = deserialize(&md).map_err(|error| IoError { message: format!("MdIntoDag: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
