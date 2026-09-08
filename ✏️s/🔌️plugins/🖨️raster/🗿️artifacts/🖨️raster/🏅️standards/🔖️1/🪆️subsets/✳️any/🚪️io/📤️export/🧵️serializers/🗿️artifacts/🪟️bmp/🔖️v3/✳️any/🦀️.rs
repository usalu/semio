//! 📤️ raster → bmp — REAL. The document's visible layer stack is flattened to one canonical
//! RGBA8 canvas by `raster_composite_image`, handed to stdio's own registered
//! `s.stdio.semio/v1/image` → `s.stdio.bmp` serializer, and written by stdio's own byte encoder.
//! This plugin owns no bmp byte codec and never will.
//!
//! 🧾️ BMP v3 carries no alpha channel: stdio's own `encode_bmp` writes 24bpp `BI_RGB` rows and drops alpha. That loss is the FORMAT's, documented by that codec, not a shortcut taken here.
use crate::artifacts::raster::io::{raster_composite_image, semio_image_to_format, BMP_DIALECT};
use crate::artifacts::raster::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let image = raster_composite_image(snapshot).map_err(|reason| format!("bmp export not available for this raster document: {reason}"))?;
    let target: semio_s_artifact_stdio_bmp::BmpSnapshot = semio_image_to_format(&image, BMP_DIALECT)?;
    semio_s_artifact_stdio_bmp::io::encode_bmp(&target)
}
