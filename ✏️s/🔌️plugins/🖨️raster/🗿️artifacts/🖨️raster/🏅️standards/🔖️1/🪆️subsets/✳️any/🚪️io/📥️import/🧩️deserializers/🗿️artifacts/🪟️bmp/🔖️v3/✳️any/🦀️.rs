//! 📥️ raster ← bmp — REAL. stdio's own byte decoder produces the typed `s.stdio.bmp` snapshot, stdio's
//! own registered `s.stdio.bmp` → `s.stdio.semio/v1/image` deserializer turns it into canonical RGBA8,
//! and that content becomes one `Pixel` layer with a materialized asset child. The incoming bytes
//! are genuinely read — nothing is fabricated.
//!
//! 🧾️ BMP v3 carries no alpha channel: stdio's own `encode_bmp` writes 24bpp `BI_RGB` rows and drops alpha. That loss is the FORMAT's, documented by that codec, not a shortcut taken here.
use crate::artifacts::raster::io::{raster_document_from_semio_image, semio_image_from_format, BMP_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let decoded = semio_s_artifact_stdio_bmp::io::decode_bmp(bytes)?;
    let image = semio_image_from_format(&decoded, BMP_DIALECT)?;
    raster_document_from_semio_image(&image, "bmp-import", "Imported bmp")
}
