//! 🚪️ note <- dwg — foreign `Deserializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Maps `Line`/`LwPolyline`
//! entities onto ink blocks and `Text` entities onto text blocks — a real, honest domain mapping
//! over typed `DwgGeometry` fields (not hand-rolled byte manipulation), but not full CAD fidelity,
//! so this hop is `IoFidelity::Lossy`.

use crate::artifacts::note::NoteSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::dwg::dwg_from_bytes;

pub const DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY };

pub struct DwgIntoNote;

impl Deserializer<NoteSnapshot> for DwgIntoNote {
    const FROM: Dialect = DWG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<NoteSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "DwgIntoNote: expected a binary dwg payload".to_string(), diagnostics: Vec::new() });
        };
        let drawing = dwg_from_bytes(bytes).map_err(|error| IoError { message: format!("DwgIntoNote: {error}"), diagnostics: Vec::new() })?;
        let value = crate::artifacts::note::io::note_document_json_from_dwg(&drawing).map_err(|error| IoError { message: format!("DwgIntoNote: {error}"), diagnostics: Vec::new() })?;
        let snapshot: NoteSnapshot = serde_json::from_value(value).map_err(|error| IoError { message: format!("DwgIntoNote: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
