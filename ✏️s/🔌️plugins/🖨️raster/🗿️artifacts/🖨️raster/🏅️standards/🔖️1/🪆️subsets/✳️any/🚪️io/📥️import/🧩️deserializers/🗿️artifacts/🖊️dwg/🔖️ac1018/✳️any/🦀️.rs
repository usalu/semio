//! raster <- dwg
use crate::artifacts::raster::RasterSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::schema::snapshot::decode_dwg;
use semio_s_plugin_stdio::artifacts::dwg::{dwg_from_bytes, DwgDrawing};
pub fn register() {}
// 🪦 `deserialize(from: &DwgSnapshot)` (a `.bytes`-reconstructing wrapper around `deserialize_bytes`)
// removed: stdio's `DwgSnapshot` no longer retains a raw byte blob (fully decomposed into structured
// fields), so that reconstruction is no longer possible, and the real call site
// (`🚪️io/🦀️.rs`'s composer) already calls `deserialize_bytes` directly with the encoded
// bytes, same as every sibling format deserializer in this directory (png, jpg, …) — this wrapper had
// no caller.
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let _meta = decode_dwg(bytes)?;
    let drawing: DwgDrawing = dwg_from_bytes(bytes)?;
    crate::artifacts::raster::io::raster_document_json_from_dwg(&drawing)
}
