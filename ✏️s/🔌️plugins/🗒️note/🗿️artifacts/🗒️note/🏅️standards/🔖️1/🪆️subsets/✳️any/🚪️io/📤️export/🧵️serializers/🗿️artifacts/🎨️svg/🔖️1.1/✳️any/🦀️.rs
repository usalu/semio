//! 🚪️ note -> svg — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Real bridge through stdio's
//! semio/drawing subset (`crate::standards::v1::subsets::any::io::note_document_to_svg`) — text/image/ink
//! blocks map onto real drawing nodes, table/math/group fall back to an outline rectangle, so this
//! hop is `IoFidelity::Lossy`. SVG's own native form is XML text, so the payload is `Text`, never a
//! raw-bytes `Binary` wrapper (the class of bug this ticket's carrier-law fix targets).

use crate::NoteSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct NoteIntoSvg;

impl Serializer<NoteSnapshot> for NoteIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &NoteSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let (svg, _width, _height) = crate::standards::v1::subsets::any::io::note_document_to_svg(from).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("NoteIntoSvg: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Text(svg)))
    }
}
