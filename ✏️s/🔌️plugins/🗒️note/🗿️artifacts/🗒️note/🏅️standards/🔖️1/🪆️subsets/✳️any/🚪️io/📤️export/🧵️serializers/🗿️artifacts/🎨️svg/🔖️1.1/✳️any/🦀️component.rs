//! 🚪️ note -> svg — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Real bridge through stdio's
//! semio/drawing subset (`crate::artifacts::note::io::note_document_to_svg`) — text/image/ink
//! blocks map onto real drawing nodes, table/math/group fall back to an outline rectangle, so this
//! hop is `IoFidelity::Lossy`. SVG's own native form is XML text, so the payload is `Text`, never a
//! raw-bytes `Binary` wrapper (the class of bug this ticket's carrier-law fix targets).

use crate::artifacts::note::NoteSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct NoteIntoSvg;

impl Serializer<NoteSnapshot> for NoteIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &NoteSnapshot) -> IoResult<IoPayload> {
        let (svg, _width, _height) = crate::artifacts::note::io::note_document_to_svg(from).map_err(|error| IoError { message: format!("NoteIntoSvg: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Text(svg)))
    }
}
