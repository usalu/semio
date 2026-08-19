//! 🚪️ note -> dwg — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Bridges through an SVG
//! rendering, then `semio_framework_os::svg_to_dwg_bytes` — an indirect, geometry-approximating
//! path, so this hop is `IoFidelity::Lossy`.

use crate::artifacts::note::NoteSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::{decode_dwg, encode_dwg};

pub const DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY };

pub struct NoteIntoDwg;

impl Serializer<NoteSnapshot> for NoteIntoDwg {
    const INTO: Dialect = DWG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &NoteSnapshot) -> IoResult<IoPayload> {
        let (svg, _w, _h) = crate::artifacts::note::io::note_document_to_svg(from).map_err(|error| IoError { message: format!("NoteIntoDwg: svg bridge: {error}"), diagnostics: Vec::new() })?;
        let raw = semio_framework_os::svg_to_dwg_bytes(&svg).map_err(|error| IoError { message: format!("NoteIntoDwg: svg_to_dwg: {error}"), diagnostics: Vec::new() })?;
        let drawing = decode_dwg(&raw).map_err(|error| IoError { message: format!("NoteIntoDwg: decode: {error}"), diagnostics: Vec::new() })?;
        let bytes = encode_dwg(&drawing).map_err(|error| IoError { message: format!("NoteIntoDwg: encode: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
