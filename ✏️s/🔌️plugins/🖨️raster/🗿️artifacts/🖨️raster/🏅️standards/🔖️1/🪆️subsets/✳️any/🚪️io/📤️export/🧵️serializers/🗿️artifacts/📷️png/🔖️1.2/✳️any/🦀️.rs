//! 📤️ raster → png — REAL. The document's visible layer stack is flattened to one canonical
//! RGBA8 canvas by `raster_composite_image`, handed to stdio's own registered
//! `s.stdio.semio/v1/image` → `s.stdio.png` serializer, and written by stdio's own byte encoder.
//! This plugin owns no png byte codec and never will.
//!
//! 🧾️ `encode_png` always re-emits canonical RGBA8 (color type 6, bit depth 8), so this hop is lossless in both directions.
use crate::io::{raster_composite_image, semio_image_to_format, PNG_DIALECT};
use crate::RasterSnapshot;
pub fn register() {}
pub fn serialize_bytes(snapshot: &RasterSnapshot) -> Result<Vec<u8>, String> {
    let image = raster_composite_image(snapshot).map_err(|reason| format!("png export not available for this raster document: {reason}"))?;
    let target: semio_s_artifact_stdio_png::PngSnapshot = semio_image_to_format(&image, PNG_DIALECT)?;
    semio_s_artifact_stdio_png::io::encode_png(&target)
}
