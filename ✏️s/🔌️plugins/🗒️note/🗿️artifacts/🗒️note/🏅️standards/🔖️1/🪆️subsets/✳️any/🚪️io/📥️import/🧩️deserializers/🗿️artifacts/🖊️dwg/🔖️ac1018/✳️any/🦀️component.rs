//! note <- dwg
use crate::artifacts::note::NoteSnapshot;
use semio_framework::{dwg_from_bytes, DwgDrawing};
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::decode_dwg;
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;
pub fn register() {}
pub fn deserialize(from: &DwgSnapshot) -> Result<NoteSnapshot, String> { deserialize_bytes(&from.bytes) }
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let _meta = decode_dwg(bytes)?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes)?;
    let value = crate::artifacts::note::io::note_document_json_from_dwg(&drawing)?;
    serde_json::from_value(value).map_err(|e| e.to_string())
}
