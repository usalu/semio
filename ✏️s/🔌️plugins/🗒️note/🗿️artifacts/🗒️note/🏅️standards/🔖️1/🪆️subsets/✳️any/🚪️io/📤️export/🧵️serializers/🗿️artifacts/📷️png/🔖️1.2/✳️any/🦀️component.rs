//! note -> png
//!
//! 🩹️ `stdio_gap`/foreign-lag fix (not part of this wave's svg/dwg-pattern scope — see
//! `w5b--report.md`): `PngSnapshot` was restructured from a `RasterImage{width,height,rgba}`
//! wrapper into a real chunk-level model (`width`/`height`/`pixels` directly on the snapshot,
//! plus IHDR/PLTE/tRNS/ancillary fields) by a concurrent stdio wave. Fixed as a minimal
//! lagging-call-site update — the canonical always-8bit-RGBA `pixels` payload this leaf already
//! built is exactly what `PngSnapshot.pixels` wants, just without the old wrapper struct.
use crate::artifacts::note::NoteSnapshot;
use semio_s_plugin_stdio::artifacts::png::engine::{encode_png, empty_png_snapshot};
pub fn register() {}
pub fn serialize(snapshot: &NoteSnapshot) -> Result<semio_s_plugin_stdio::artifacts::png::PngSnapshot, String> {
    let (w, h) = crate::artifacts::note::engine::note_document_bounds(snapshot);
    let width = w.max(1); let height = h.max(1);
    let mut rgba = vec![255u8; (width as usize) * (height as usize) * 4];
    for px in rgba.chunks_mut(4) { px[3] = 255; }
    let mut snap = empty_png_snapshot();
    snap.width = width;
    snap.height = height;
    snap.pixels = rgba;
    Ok(snap)
}
pub fn serialize_bytes(snapshot: &NoteSnapshot) -> Result<Vec<u8>, String> { encode_png(&serialize(snapshot)?) }
