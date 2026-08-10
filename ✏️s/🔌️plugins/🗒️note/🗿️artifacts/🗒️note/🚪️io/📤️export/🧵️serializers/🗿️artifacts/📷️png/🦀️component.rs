//! note -> png
use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::png::engine::{encode_png, empty_png_snapshot};
use semio_s_plugin_stdio::artifacts::png::schema::snapshot::RasterImage;
pub fn register() {}
pub fn serialize(snapshot: &NoteSnapshot) -> Result<semio_s_plugin_stdio::artifacts::png::PngSnapshot, String> {
    let (w, h) = crate::artifacts::note::engine::note_document_bounds(snapshot);
    let width = w.max(1); let height = h.max(1);
    let mut rgba = vec![255u8; (width as usize) * (height as usize) * 4];
    for px in rgba.chunks_mut(4) { px[3] = 255; }
    let mut snap = empty_png_snapshot();
    snap.image = RasterImage { width, height, rgba };
    Ok(snap)
}
pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> { encode_png(&serialize(snapshot)?) }
