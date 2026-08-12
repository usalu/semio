//! note -> dwg
use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::{decode_dwg, encode_dwg};
use semio_s_plugin_stdio::artifacts::dwg::DwgSnapshot;
pub fn register() {}
pub fn serialize(snapshot: &NoteSnapshot) -> Result<DwgSnapshot, String> {
    decode_dwg(&serialize_bytes(snapshot)?)
}
pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> {
    let (svg, _w, _h) = crate::artifacts::note::engine::note_document_to_svg(snapshot)?;
    let bytes = semio_framework_os::svg_to_dwg_bytes(&svg)?;
    encode_dwg(&decode_dwg(&bytes)?)
}
