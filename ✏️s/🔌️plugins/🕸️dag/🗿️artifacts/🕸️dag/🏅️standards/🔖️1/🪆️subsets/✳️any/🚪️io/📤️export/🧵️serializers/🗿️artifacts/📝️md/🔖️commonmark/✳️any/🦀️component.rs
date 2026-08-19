//! 🚪️ dag -> md — foreign `Serializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Writes this plugin's own
//! `.dag` DSL text as MD's text body (round-trips through `stdio.md`'s block model on the way
//! back in — see the sibling `Deserializer`), so this hop is `IoFidelity::Canonical`.

use crate::artifacts::dag::DagSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub async fn serialize(from: &DagSnapshot) -> Result<MdSnapshot, store::PackError> {
    let _ = STDIO_MD_DOCUMENT_SCHEMA;
    Ok(MdSnapshot::from_text(&<DagSnapshot as store::ArtifactDsl>::print_dsl(from)))
}

pub struct DagIntoMd;

impl Serializer<DagSnapshot> for DagIntoMd {
    const INTO: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn serialize(from: &DagSnapshot) -> IoResult<IoPayload> {
        let md = serialize(from).map_err(|error| semio_framework::io_schema::IoError { message: format!("DagIntoMd: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(<MdSnapshot as store::ArtifactPack>::encode_pack(&md))))
    }
}
