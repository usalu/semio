//! 📥️ raster ← jpg — REAL. stdio's own byte decoder produces the typed `s.stdio.jpg` snapshot, stdio's
//! own registered `s.stdio.jpg` → `s.stdio.semio/v1/image` deserializer turns it into canonical RGBA8,
//! and that content becomes one `Pixel` layer with a materialized asset child. The incoming bytes
//! are genuinely read — nothing is fabricated.
//!
//! 🧾️ JPEG is lossy by construction and carries no alpha; stdio's own codec forces alpha opaque on decode and re-quantizes on encode. Both are the FORMAT's losses, documented by that codec.
use crate::artifacts::raster::io::{raster_document_from_semio_image, semio_image_from_format, JPG_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<RasterSnapshot, String> {
    let decoded = semio_s_artifact_stdio_jpg::io::decode_jpg(bytes).map_err(|error| format!("{error:?}"))?;
    let image = semio_image_from_format(&decoded, JPG_DIALECT)?;
    raster_document_from_semio_image(&image, "jpg-import", "Imported jpg")
}
