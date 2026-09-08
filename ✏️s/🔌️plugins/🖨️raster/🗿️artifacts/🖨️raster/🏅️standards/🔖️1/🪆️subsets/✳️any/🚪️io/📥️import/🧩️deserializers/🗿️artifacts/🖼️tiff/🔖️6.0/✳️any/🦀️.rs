//! 📥️ raster ← tiff — REAL. stdio's own byte decoder produces the typed `s.stdio.tiff` snapshot, stdio's
//! own registered `s.stdio.tiff` → `s.stdio.semio/v1/image` deserializer turns it into canonical RGBA8,
//! and that content becomes one `Pixel` layer with a materialized asset child. The incoming bytes
//! are genuinely read — nothing is fabricated.
//!
//! 🧾️ stdio's TIFF codec decodes/encodes IFD 0 as canonical RGBA8 strips.
use crate::io::{raster_document_from_semio_image, semio_image_from_format, TIFF_DIALECT};
use crate::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let decoded = semio_s_artifact_stdio_tiff::io::decode_tiff(bytes)?;
    let image = semio_image_from_format(&decoded, TIFF_DIALECT)?;
    raster_document_from_semio_image(&image, "tiff-import", "Imported tiff")
}
