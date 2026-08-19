//! 🚪️ dag <- svg — foreign `Deserializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Reads the svg document's own
//! text content and parses it as this plugin's `.dag` DSL — a pre-migration bridge, not a real
//! svg-shape import; the sibling `Serializer` does NOT invert this (best-effort structural
//! reinterpretation instead), so the round trip is not lossless: `IoFidelity::Lossy`.

use crate::artifacts::dag::DagSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::svg::{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::xml::schema::snapshot::xml_document_to_text;

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub async fn deserialize(from: &SvgSnapshot) -> Result<DagSnapshot, store::TextError> {
    let _ = STDIO_SVG_DOCUMENT_SCHEMA;
    let text = xml_document_to_text(&from.doc);
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(&text)
}

pub struct SvgIntoDag;

impl Deserializer<DagSnapshot> for SvgIntoDag {
    const FROM: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<DagSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "SvgIntoDag: expected a binary svg payload".to_string(), diagnostics: Vec::new() });
        };
        let svg = <SvgSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("SvgIntoDag: svg decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = deserialize(&svg).map_err(|error| IoError { message: format!("SvgIntoDag: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
