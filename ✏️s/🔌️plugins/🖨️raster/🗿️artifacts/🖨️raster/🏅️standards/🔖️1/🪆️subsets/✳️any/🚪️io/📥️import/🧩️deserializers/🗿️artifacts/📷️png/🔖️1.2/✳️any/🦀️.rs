//! 📥️ raster ← png — REAL. stdio's own byte decoder produces the typed `s.stdio.png` snapshot, stdio's
//! own registered `s.stdio.png` → `s.stdio.semio/v1/image` deserializer turns it into canonical RGBA8,
//! and that content becomes one `Pixel` layer with a materialized asset child. The incoming bytes
//! are genuinely read — nothing is fabricated.
//!
//! 🧾️ `encode_png` always re-emits canonical RGBA8 (color type 6, bit depth 8), so this hop is lossless in both directions.
use crate::artifacts::raster::io::{raster_document_from_semio_image, semio_image_from_format, PNG_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let decoded = semio_s_plugin_stdio::artifacts::png::io::decode_png(bytes)?;
    let image = semio_image_from_format(&decoded, PNG_DIALECT)?;
    raster_document_from_semio_image(&image, "png-import", "Imported png")
}
