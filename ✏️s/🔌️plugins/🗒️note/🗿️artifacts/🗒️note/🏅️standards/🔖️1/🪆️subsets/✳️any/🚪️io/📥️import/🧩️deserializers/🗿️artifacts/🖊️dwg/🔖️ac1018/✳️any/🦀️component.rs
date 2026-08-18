//! note <- dwg
use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::{decode_dwg, encode_dwg};
use semio_s_plugin_stdio::artifacts::dwg::{dwg_from_bytes, DwgDrawing, DwgSnapshot};
pub fn register() {}
/// 🔁️ `DwgSnapshot` carries a structured `drawing: DwgLogicalDrawing`, not raw bytes — round-trips
/// through `encode_dwg` to reach the byte-oriented `deserialize_bytes` below (mirrors the export
/// side's own `serialize`/`serialize_bytes` split, which already goes bytes-first).
pub fn deserialize(from: &DwgSnapshot) -> Result<NoteSnapshot, String> {
    let bytes = encode_dwg(from).map_err(|error| error.to_string())?;
    deserialize_bytes(&bytes)
}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<NoteSnapshot, String> {
    let _meta = decode_dwg(bytes)?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes)?;
    let value = crate::artifacts::note::io::note_document_json_from_dwg(&drawing)?;
    serde_json::from_value(value).map_err(|e| e.to_string())
}
