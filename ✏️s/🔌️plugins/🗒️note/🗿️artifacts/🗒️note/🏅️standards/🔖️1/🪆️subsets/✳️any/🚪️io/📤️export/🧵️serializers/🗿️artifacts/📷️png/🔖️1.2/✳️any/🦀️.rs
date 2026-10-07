//! 🚪️ note -> png — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Rasterizes to an opaque white
//! canvas sized to the document bounds — no block content is actually painted (unchanged behaviour
//! from the pre-migration free function) — an honest `IoFidelity::Lossy` hop.

use crate::standards::v1::subsets::any::io::note_document_bounds;
use crate::NoteSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_pixels::{encode_png, RasterImage};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct NoteIntoPng;

impl Serializer<NoteSnapshot> for NoteIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &NoteSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let (w, h) = note_document_bounds(from);
        let width = w.max(1);
        let height = h.max(1);
        let mut rgba = vec![255u8; (width as usize) * (height as usize) * 4];
        for px in rgba.chunks_mut(4) {
            px[3] = 255;
        }
        let bytes = encode_png(&RasterImage { width, height, pixels: rgba }).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("NoteIntoPng: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
