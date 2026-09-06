//! 🚪️ note -> png — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Rasterizes to an opaque white
//! canvas sized to the document bounds — no block content is actually painted (unchanged behaviour
//! from the pre-migration free function) — an honest `IoFidelity::Lossy` hop.

use crate::artifacts::note::io::note_document_bounds;
use crate::artifacts::note::NoteSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::png::io::encode_png;
use semio_s_plugin_stdio::artifacts::png::schema::empty_png_snapshot;

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct NoteIntoPng;

impl Serializer<NoteSnapshot> for NoteIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &NoteSnapshot) -> IoResult<IoPayload> {
        let (w, h) = note_document_bounds(from);
        let width = w.max(1);
        let height = h.max(1);
        let mut rgba = vec![255u8; (width as usize) * (height as usize) * 4];
        for px in rgba.chunks_mut(4) {
            px[3] = 255;
        }
        let mut snapshot = empty_png_snapshot();
        snapshot.width = width;
        snapshot.height = height;
        snapshot.pixels = rgba;
        let bytes = encode_png(&snapshot).map_err(|error| IoError { message: format!("NoteIntoPng: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
